use diesel::prelude::*;
use diesel::result::QueryResult;
use crate::models::arc::arc::Arc as T;
use crate::schema::arcs::table as MyTable;
use uuid::Uuid as MyId;

impl crate::traits::db_crud::DBCrud<MyId, T, T, T> for T {
    db_crud!(crate::db_connection, MyTable, MyId, T, T, T,);
}
