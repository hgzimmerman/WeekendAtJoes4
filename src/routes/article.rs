use rocket::Route;
use rocket_contrib::Json;
use super::Routable;
use diesel;
use diesel::RunQueryDsl;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use db::Conn;

use rocket::response::status::Custom;
use rocket::http::Status;
use db::article::*;
use requests_and_responses::article::*;
// use routes::DatabaseError;
use rocket::response::status::NoContent;
use routes::WeekendAtJoesError;

// TODO: change the return type of this to Result<Json<Article>, Custom<>>
// return a custom 404 or a custom 500 depending on the error type
#[get("/<article_id>", rank=0)]
fn get_article(article_id: i32, conn: Conn) -> Option<Json<Article>> {
    
    match Article::get_article_by_id(article_id, &conn) {
        Ok(article_option) => article_option.and_then(|article| Some(Json(article))),
        Err(e) => {
            warn!("Getting article failed for reason: {:?}", e);
            None
        }
    }
}

#[post("/", data = "<new_article>")]
fn create_article(new_article: Json<NewArticleRequest>, conn: Conn) -> Result<Json<Article>, Custom<&'static str>> {

    match Article::create_article(new_article.into_inner(), &conn) {
        Ok(article) => (Ok(Json(article))),
        Err(e) => Err(Custom(Status::InternalServerError, "DB Error"))
    }
}

/// Updates the article.
// TODO: Consider not exposing this directly, and instead create update methods for publishing and editing.
#[put("/", data = "<update_article>")]
fn update_article(update_article: Json<Article>, db_conn: Conn) -> Json<Article> {
    use schema::articles;

    let article: Article = update_article.into_inner();

    let updated_article: Article = diesel::update(articles::table)
        .set(&article)
        .get_result(&*db_conn)
        .expect("Failed to insert");
    
    Json(updated_article)
}

// TODO, test this interface
#[delete("/<article_id>")]
fn delete_article(article_id: i32, conn: Conn) -> Result<NoContent, WeekendAtJoesError> {
    Article::delete_article(article_id, &conn) 
}

// TODO, test this interface
#[put("/publish/<article_id>")]
fn publish_article(article_id: i32, conn: Conn) -> Result<NoContent, WeekendAtJoesError> {
    Article::publish_article(article_id, &conn)
}

// Export the ROUTES and their path
pub fn article_routes() -> Vec<Route> {
    routes![create_article, update_article, get_article, delete_article]
}


impl Routable for Article {
    const ROUTES: &'static Fn() -> Vec<Route> = &||routes![create_article, update_article, get_article, delete_article];
    const PATH: &'static str = "/article/";
}
