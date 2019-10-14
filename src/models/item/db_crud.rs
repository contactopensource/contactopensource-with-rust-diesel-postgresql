use diesel::prelude::*;
use diesel::result::QueryResult;
use crate::models::item::item::Item as T;

impl crate::traits::db_crud::DBCrud<crate::schema::items::table, uuid::Uuid, T, T, T> for T {
    db_crud!(
        crate::schema::items::table,
        uuid::Uuid,
        crate::models::item::item::Item,
        crate::models::item::item::Item,
        crate::models::item::item::Item,
    );
}
