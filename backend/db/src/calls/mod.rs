//! Calls to the database.

pub mod answer;
pub mod article;
pub mod auth;
pub mod bucket;
pub mod chat;
pub mod forum;
pub mod message;
pub mod post;
pub mod question;
pub mod thread;
pub mod user;

use diesel::{
    associations::HasTable,
    delete,
    dsl::Find,
    helper_types::Update,
    insertable::Insertable,
    pg::{
        Pg,
        PgConnection,
    },
    query_builder::{
        AsChangeset,
        AsQuery,
        DeleteStatement,
        InsertStatement,
        IntoUpdateTarget,
        QueryFragment,
        QueryId,
    },
    query_dsl::{
        filter_dsl::FindDsl,
        LoadQuery,
        RunQueryDsl,
    },
    query_source::{
        QuerySource,
        Queryable,
        Table,
    },
    result::Error as DieselError,
    sql_types::HasSqlType,
    Expression,
};
use error::Error;
use typename::TypeName;
use uuid::Uuid;

pub mod prelude {
    pub use super::{
        create_row,
        delete_row,
        get_row,
        get_rows,
        handle_err,
        update_row,
    };
}

pub fn handle_err<T: TypeName>(error: DieselError) -> Error {
    match error {
        DieselError::NotFound => Error::NotFound {
            type_name: T::type_name(),
        },
        _ => Error::DatabaseError(Some(format!("{:?}", error))), // This gives some insight into what the internal state of the app is. Set this to none when this enters production.
    }
}

/// Generic function for getting a whole row from a given table.
#[inline(always)]
pub fn get_row<'a, Model, Table>(table: Table, uuid: Uuid, conn: &PgConnection) -> Result<Model, Error>
where
    Table: FindDsl<Uuid>,
    Find<Table, Uuid>: LoadQuery<PgConnection, Model>,
    Model: TypeName,
{
    table.find(uuid).get_result::<Model>(conn).map_err(handle_err::<Model>)
}

#[inline(always)]
pub fn get_rows<'a, Model, Table>(table: Table, conn: &PgConnection) -> Result<Vec<Model>, Error>
where
    Table: RunQueryDsl<Model> + LoadQuery<PgConnection, Model>,
    Model: TypeName,
{
    table.load::<Model>(conn).map_err(handle_err::<Model>)
}

/// Generic function for deleting a row from a given table.
#[inline(always)]
pub fn delete_row<'a, Model, Tab>(table: Tab, uuid: Uuid, conn: &PgConnection) -> Result<Model, Error>
where
    Tab: FindDsl<Uuid> + Table,
    <Tab as FindDsl<Uuid>>::Output: IntoUpdateTarget,
    Pg: HasSqlType<<<<<Tab as FindDsl<Uuid>>::Output as HasTable>::Table as Table>::AllColumns as Expression>::SqlType>,
    <<<Tab as FindDsl<Uuid>>::Output as HasTable>::Table as Table>::AllColumns: QueryId,
    <<<Tab as FindDsl<Uuid>>::Output as HasTable>::Table as Table>::AllColumns: QueryFragment<Pg>,
    DeleteStatement<
        <<Tab as FindDsl<Uuid>>::Output as HasTable>::Table,
        <<Tab as FindDsl<Uuid>>::Output as IntoUpdateTarget>::WhereClause,
    >: LoadQuery<PgConnection, Model>,
    Model: TypeName,
{
    delete(table.find(uuid))
        .get_result::<Model>(conn)
        .map_err(handle_err::<Model>)
}

/// Generic function for updating a row for a given table with a given changeset.
#[inline(always)]
pub fn update_row<'a, Model, Chg, Tab>(table: Tab, changeset: Chg, conn: &PgConnection) -> Result<Model, Error>
where
    Chg: AsChangeset<Target = <Tab as HasTable>::Table>,
    Tab: QuerySource + IntoUpdateTarget,
    Update<Tab, Chg>: LoadQuery<PgConnection, Model>,
    Model: TypeName,
{
    diesel::update(table)
        .set(changeset)
        .get_result::<Model>(conn)
        .map_err(handle_err::<Model>)
}

/// Generic function for creating a row for a given table with a given "new" struct for that row type.
#[inline(always)]
pub fn create_row<Model, NewModel, Tab>(table: Tab, insert: NewModel, conn: &PgConnection) -> Result<Model, Error>
where
    NewModel: Insertable<Tab>,
    InsertStatement<Tab, NewModel>: AsQuery,
    Pg: HasSqlType<<InsertStatement<Tab, NewModel> as AsQuery>::SqlType>,
    InsertStatement<Tab, <NewModel as Insertable<Tab>>::Values>: AsQuery,
    Model: Queryable<<InsertStatement<Tab, <NewModel as Insertable<Tab>>::Values> as AsQuery>::SqlType, Pg>,
    Pg: HasSqlType<<InsertStatement<Tab, <NewModel as Insertable<Tab>>::Values> as AsQuery>::SqlType>,
    <InsertStatement<Tab, <NewModel as Insertable<Tab>>::Values> as AsQuery>::Query: QueryId,
    <InsertStatement<Tab, <NewModel as Insertable<Tab>>::Values> as AsQuery>::Query: QueryFragment<Pg>,
    Model: TypeName,
{
    insert
        .insert_into(table)
        .get_result::<Model>(conn)
        .map_err(handle_err::<Model>)
}

//fn row_exists<'a, Model, Tab>(table: Tab, uuid: Uuid, conn: &PgConnection) -> Result<bool, Error>
//where
//    Tab: FindDsl<Uuid> + Table,
//    <Tab as FindDsl<uuid::Uuid>>::Output: SelectQuery,
//    <Tab as FindDsl<uuid::Uuid>>::Output: QueryId,
//    <Tab as FindDsl<uuid::Uuid>>::Output: ValidSubselect<()>,
//    <Tab as FindDsl<Uuid>>::Output : SelectDsl<Exists<<Tab as FindDsl<Uuid>>::Output>>,
//    Find<Tab, Uuid>: LoadQuery<PgConnection, Model>,
////    SelectStatement<(), Select=Exists<<Table as FindDsl<Uuid>>::Output>>: QueryFragment<Pg>//, SelectClause<Exists<<Table as FindDsl<uuid::Uuid>>::Output>>>: QueryFragment<Pg>
//{
////    table.find(uuid)
////        .select(exists)
//
////    select(exists(table.filter(table.primary_key().eq(uuid))))
//    table
////        .find(uuid)
//        .count()
//        .get_result::<i32>(conn)
//}
//
//use diesel::QueryDsl;
