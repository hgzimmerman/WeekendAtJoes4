use error::Error;
use warp::{
    filters::BoxedFilter,
    reply::Reply,
    Filter,
};
//use crate::db_integration::s.db.clone();
//use db::Conn;
use crate::{
    logging::{
        log_attach,
        HttpMethod,
    },
    state::{
        jwt::normal_user_filter,
        State,
    },
    util::{
        convert_and_json,
        convert_vector_and_json,
        json_body_filter,
        query_uuid,
    },
    uuid_integration::{
        uuid_filter,
        uuid_wrap_filter,
    },
};
use db::{
    question::{
        NewQuestion,
        QuestionData,
    },
    Question,
};
use identifiers::{
    bucket::BucketUuid,
    question::QuestionUuid,
    user::UserUuid,
};
use pool::PooledConn;
use uuid::Uuid;
use wire::question::{
    NewQuestionRequest,
    QuestionResponse,
};
use crate::state::jwt::optional_normal_user_filter;

pub fn question_api(s: &State) -> BoxedFilter<(impl Reply,)> {
    info!("Attaching Question API");
    let api = get_questions_for_bucket(s)
        .or(get_question(s))
        .or(create_question(s))
        .or(get_random_question(s))
        .or(get_questions_for_bucket(s))
        .or(delete_question(s))
        .or(put_question_back_in_bucket(s))
        .or(questions_in_bucket(s))
        .or(favorite_question(s))
        .or(unfavorite_question(s))
        .or(get_favorite_questions(s))
    ;

    warp::path("question").and(api).with(warp::log("question")).boxed()
}

pub fn get_questions_for_bucket(s: &State) -> BoxedFilter<(impl Reply,)> {
    log_attach(HttpMethod::Get, "question?bucket_uuid=<uuid>");

    warp::get2()
        .and(query_uuid("bucket_uuid"))
        .and(s.db.clone())
        .and_then(|bucket_uuid: Uuid, conn: PooledConn| {
            let bucket_uuid = BucketUuid(bucket_uuid);
            Question::get_questions_for_bucket(bucket_uuid, &conn)
                .map(convert_vector_and_json::<QuestionData, QuestionResponse>)
                .map_err(Error::simple_reject)
        })
        .boxed()
}

fn get_random_question(s: &State) -> BoxedFilter<(impl Reply,)> {
    //    log_attach(HttpMethod::Post, "bucket/");
    log_attach(HttpMethod::Get, "question/random_question?bucket_uuid=<uuid>");

    warp::get2()
        .and(warp::path("random_question"))
        .and(query_uuid("bucket_uuid"))
        .and(s.db.clone())
        .and_then(|bucket_uuid: Uuid, conn: PooledConn| {
            let bucket_uuid = BucketUuid(bucket_uuid);
            Question::get_random_question(bucket_uuid, &conn)
                .map(convert_and_json::<QuestionData, QuestionResponse>)
                .map_err(Error::simple_reject)
        })
        .boxed()
}
fn get_question(s: &State) -> BoxedFilter<(impl Reply,)> {
    log_attach(HttpMethod::Get, "question/<uuid>");

    warp::get2()
        .and(uuid_wrap_filter())
        .and(s.db.clone())
        .and_then(|question_uuid: QuestionUuid, conn: PooledConn| {
            Question::get_full_question(question_uuid, &conn)
                .map(convert_and_json::<QuestionData, QuestionResponse>)
                .map_err(Error::simple_reject)
        })
        .boxed()
}

// TODO there should be a variant that doesn't require auth.
fn create_question(s: &State) -> BoxedFilter<(impl Reply,)> {
    log_attach(HttpMethod::Post, "question/");

    warp::post2()
        .and(json_body_filter(12))
        .and(optional_normal_user_filter(s))
        .and(s.db.clone())
        .and_then(|request: NewQuestionRequest, user_uuid: Option<UserUuid>, conn: PooledConn| {
//            let bucket_uuid: BucketUuid = request.bucket_uuid;
//            let is_approved = Bucket::is_user_approved(user_uuid, bucket_uuid, &conn);
//            if !is_approved {
//                return Error::BadRequest.reject();
//            }

            let new_question: NewQuestion = NewQuestion::attach_user_id(request, user_uuid);

            Question::create_data(new_question, &conn)
                .map(convert_and_json::<QuestionData, QuestionResponse>)
                .map_err(Error::simple_reject)
        })
        .boxed()
}

fn delete_question(s: &State) -> BoxedFilter<(impl Reply,)> {
    log_attach(HttpMethod::Delete, "question/<uuid>");

    warp::delete2()
        .and(uuid_filter())
        .and(normal_user_filter(s))
        .and(s.db.clone())
        .and_then(|question_uuid: Uuid, _user_uuid: UserUuid, conn: PooledConn| {
            let question_uuid = QuestionUuid(question_uuid);
            Question::delete_question(question_uuid.clone(), &conn)
                .map_err(Error::simple_reject)
                .map(|_| warp::reply::json(&question_uuid))
        })
        .boxed()
}

fn put_question_back_in_bucket(s: &State) -> BoxedFilter<(impl Reply,)> {
    log_attach(HttpMethod::Put, "question/<uuid>/into_bucket/");

    warp::put2()
        .and(uuid_filter())
        .and(warp::path("into_bucket"))
        .and(normal_user_filter(s))
        .and(s.db.clone())
        .and_then(|question_uuid: Uuid, _user_uuid: UserUuid, conn: PooledConn| {
            let question_uuid = QuestionUuid(question_uuid);
            Question::put_question_in_bucket(question_uuid, &conn)
                .map_err(Error::simple_reject)
                .map(|_| warp::reply::json(&question_uuid))
        })
        .boxed()
}

fn questions_in_bucket(s: &State) -> BoxedFilter<(impl Reply,)> {
    log_attach(HttpMethod::Get, "question/quantity_in_bucket?bucket_uuid=<uuid>");

    warp::get2()
        .and(warp::path("quantity_in_bucket"))
        .and(query_uuid("bucket_uuid"))
        .and(s.db.clone())
        .and_then(|bucket_uuid: Uuid, conn: PooledConn| {
            let bucket_uuid = BucketUuid(bucket_uuid);
            Question::get_number_of_questions_in_bucket(bucket_uuid, &conn)
                .map(convert_and_json::<i64, i64>)
                .map_err(Error::simple_reject)
        })
        .boxed()
}


fn favorite_question(s: &State) -> BoxedFilter<(impl Reply,)> {

    const FAVORITE: &str = "favorite";
    log_attach(HttpMethod::Put, &format!("question/{}/<question_uuid>", FAVORITE));
    warp::put2()
        .and(warp::path("favorite"))
        .and(uuid_wrap_filter())
        .and(normal_user_filter(s))
        .and(s.db.clone())
        .and_then(|question_uuid: QuestionUuid, user_uuid: UserUuid, conn: PooledConn| {
            Question::favorite_question(question_uuid, user_uuid, &conn)
                .map(convert_and_json::<(), ()>)
                .map_err(Error::simple_reject)
        })
        .boxed()
}
fn unfavorite_question(s: &State) -> BoxedFilter<(impl Reply,)> {
    const UNFAVORITE: &str = "unfavorite";
    log_attach(HttpMethod::Put, "question/unfavorite/<question_uuid>");
    warp::put2()
        .and(warp::path(UNFAVORITE))
        .and(uuid_wrap_filter())
        .and(normal_user_filter(s))
        .and(s.db.clone())
        .and_then(|question_uuid: QuestionUuid, user_uuid: UserUuid, conn: PooledConn| {
            Question::unfavorite_question(question_uuid, user_uuid, &conn)
                .map(convert_and_json::<(), ()>)
                .map_err(Error::simple_reject)
        })
        .boxed()
}

fn get_favorite_questions(s: &State) -> BoxedFilter<(impl Reply,)> {
    log_attach(HttpMethod::Put, "question/favorites");
    warp::get2()
        .and(warp::path("favorites"))
        .and(normal_user_filter(s))
        .and(s.db.clone())
        .and_then(|user_uuid: UserUuid, conn: PooledConn|{
            Question::get_favorite_questions(user_uuid, &conn)
                .map(convert_vector_and_json::<QuestionData, QuestionResponse>)
                .map_err(Error::simple_reject)
        })
        .boxed()
}