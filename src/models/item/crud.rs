use ::diesel::{prelude::*, pg::PgConnection as Connection, result::QueryResult};
use crate::types::id::Id;

use crate::schema::items as s;
use s::table as table;
use crate::models::item as t;
use t::item::Item as T;

impl T {

    /// DB create; this uses `.execute`.
    pub fn db_create(
        connection: &Connection,
        x: &T
    ) -> QueryResult<usize> {
        diesel::insert_into(table)
            .values(x)
            .execute(connection)
    }

    /// DB create; this uses `.get_results`.
    pub fn db_create_with_result(
        connection: &Connection,
        x: &T
    ) -> Result<T, ::diesel::result::Error> {
        diesel::insert_into(table)
            .values(x)
            .get_result::<T>(connection)
    }

    /// DB creates i.e. multiple values; this uses `.execute`.
    pub fn db_creates(
        connection: &Connection,
        x: &Vec<T>
    ) -> QueryResult<usize> {
        diesel::insert_into(table)
            .values(x)
            .execute(connection)
    }

    /// DB creates i.e. multiple values; this uses `.get_results`.
    pub fn db_creates_with_results(
        connection: &Connection,
        x: &Vec<T>
    ) -> Result<Vec<T>, ::diesel::result::Error> {
        diesel::insert_into(table)
            .values(x)
            .get_results::<T>(connection)
    }

    // DB read one row by id.
    pub fn db_read(
        connection: &Connection,
        id: &Id
    ) -> Result<T, ::diesel::result::Error> {
        table.find(&id).first(connection)
    }

    // DB update one row; this uses `.execute`.
    pub fn db_update(
        connection: &Connection,
        id: &Id,
        x: &T
    ) -> QueryResult<usize> {
        diesel::update(table.find(id))
        .set(x)
        .execute(connection)
    }

    // DB update one row; this uses `.get_result`
    pub fn db_update_with_result(
        connection: &Connection,
        id: &Id,
        x: &T
    ) -> Result<T, ::diesel::result::Error> {
        diesel::update(table.find(id))
            .set(x)
            .get_result::<T>(connection)
    }

    // DB delete one row by id; this uses `.execute`.
    pub fn db_delete(
        connection: &Connection,
        id: &Id
    ) -> QueryResult<usize> {
        diesel::delete(table.find(id))
            .execute(connection)
    }

    // DB delete one row by id; this uses `.get_results`.
    pub fn db_delete_with_result(
        connection: &Connection,
        id: &Id
    ) -> Result<T, ::diesel::result::Error> {
        diesel::delete(table.find(id))
            .get_result::<T>(connection)
    }

}
