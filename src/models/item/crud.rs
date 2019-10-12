use ::diesel::{prelude::*, result::QueryResult};

use crate::schema::items as s;
use s::table as table;
use crate::models::item as t;
use t::item::Item as T;

use ::diesel::PgConnection as MyConnection;
use ::diesel::result::Error as MyError;
use uuid::Uuid as MyId;
use t::item::Item as MyQueryable;
use t::item::Item as MyInsertable;
use t::item::Item as MyChangeset;

impl T {

    /// DB create; this uses `.execute`.
    ///
    /// Return the row change count as `usize`.
    ///
    /// # Example
    ///
    /// ```
    /// let result: QueryResult<usize> = db_create(connection, insertable);
    /// let count: usize = result.unwrap();
    /// ```

    pub fn db_create(
        connection: &MyConnection,
        insertable: &MyInsertable
    ) -> QueryResult<usize> {
        diesel::insert_into(table)
            .values(insertable)
            .execute(connection)
    }

    /// DB create; this uses `.get_results`.
    ///
    /// Return the result.
    ///
    /// # Example
    ///
    /// ```
    /// let result: Result<Queryable, Error> = db_create_with_result(connection, insertable);
    /// let queryable: Queryable = result.unwrap();
    /// ```

    pub fn db_create_with_result(
        connection: &MyConnection,
        insertable: &MyInsertable
    ) -> Result<MyQueryable, MyError> {
        diesel::insert_into(table)
            .values(insertable)
            .get_result::<T>(connection)
    }

    /// DB creates i.e. multiple values; this uses `.execute`.
    ///
    /// Return the row change count.
    ///
    /// # Example
    ///
    /// ```
    /// let result: QueryResult<usize> = db_creates(connection, insertables);
    /// let count: usize = result.unwrap();
    /// ```

    pub fn db_creates(
        connection: &MyConnection,
        insertables: &Vec<MyInsertable>
    ) -> QueryResult<usize> {
        diesel::insert_into(table)
            .values(insertables)
            .execute(connection)
    }

    /// DB creates i.e. multiple values; this uses `.get_results`.
    ///
    /// Return the results.
    ///
    /// # Example
    ///
    /// ```
    /// let result: Result<Queryable, Error> = db_creates_with_results(connection, insertables);
    /// let results: Vec<Queryable> = result.unwrap();
    /// ```

    pub fn db_creates_with_results(
        connection: &MyConnection,
        insertables: &Vec<MyInsertable>
    ) -> Result<Vec<MyQueryable>, MyError> {
        diesel::insert_into(table)
            .values(insertables)
            .get_results::<T>(connection)
    }

    /// DB read one row by id.
    ///
    /// Return the result.
    ///
    /// # Example
    ///
    /// ```
    /// let result: Result<Queryable, Error> = db_read(connection, id);
    /// let queryable: Queryable = result.unwrap();
    /// ```

    pub fn db_read(
        connection: &MyConnection,
        id: &MyId
    ) -> Result<MyQueryable, MyError> {
        table.find(&id).first(connection)
    }

    /// DB update one row; this uses `.execute`.
    ///
    /// Return the row change count.
    ///
    /// # Example
    ///
    /// ```
    /// let result: QueryResult<usize> = db_update(connection, id, changeset);
    /// let count: usize = result.unwrap();
    /// ```

    pub fn db_update(
        connection: &MyConnection,
        id: &MyId,
        changeset: &MyChangeset
    ) -> QueryResult<usize> {
        diesel::update(table.find(id))
        .set(changeset)
        .execute(connection)
    }

    /// DB update one row; this uses `.get_result`
    ///
    /// Return the result.
    ///
    /// # Example
    ///
    /// ```
    /// let result: Result<Queryable, MyError> = db_update_with_result(connection, id, changeset);
    /// let queryable: Queryable = result.unwrap();
    /// ```

    pub fn db_update_with_result(
        connection: &MyConnection,
        id: &MyId,
        changeset: &MyChangeset
    ) -> Result<MyQueryable, MyError> {
        diesel::update(table.find(id))
            .set(changeset)
            .get_result::<T>(connection)
    }

    /// DB delete one row by id; this uses `.execute`.
    ///
    /// Return the row change count.
    ///
    /// # Example
    ///
    /// ```
    /// let result: QueryResult<usize> = db_delete(connection, id);
    /// let count: usize = result.unwrap();
    /// ```

    pub fn db_delete(
        connection: &MyConnection,
        id: &MyId
    ) -> QueryResult<usize> {
        diesel::delete(table.find(id))
            .execute(connection)
    }

    /// DB delete one row by id; this uses `.get_results`.
    /// Returns result model.
    /// # Example
    ///
    /// ```
    /// let result: Result<Queryable, Error> = db_create_with_result(connection, id);
    /// let queryable: Queryable = result.unwrap();
    /// ```

    // DB delete one row by id; this uses `.get_results`.
    pub fn db_delete_with_result(
        connection: &MyConnection,
        id: &MyId
    ) -> Result<MyQueryable, MyError> {
        diesel::delete(table.find(id))
            .get_result::<T>(connection)
    }

}
