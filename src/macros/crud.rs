// Macros for "crud" a.k.a. database create, read, update, delete.
//
// Status: Wwork in progress, feedback appreciated.
//
// Contact: Joel Parker Henderson <joel@joelparkerhenderson.com>
//
// License: MIT
//
// # Introduction
//
// We are seeking ways to use Diesel and PostgreSQL with typical database
// capabilties for "crud" a.k.a. create, read, update, delete.
//
// We are seeing many Diesel examples and blog posts that show how to 
// do crud capabilities per table and model; so far we have not found 
// much about how to do the same kinds of capabilties across multiple 
// tables and models. We would like to write resuable generic crud code.
// 
// This file is our attempt to write Rust macros for crud capabilities.
// We are seeking feedback, guidance, and leads to other approaches.
//
// # Configuration
//
// These macros are specific to our app's usage:
//
//   * The `diesel` crate for object-relational mapping.
//
//   * The PostgreSQL database.
//
//   * The Diesel PostgreSQL connection type.
//
//   * The Diesel PostgreSQL capability to call `.get_results`.
//
//   * The `uuid` crate and `uuid::UUID` type for our table id.
// 
// You can configure these macros for your own app.
//
// To use a different Diesel connection, change this as you want:
//
// ```
// connection: &diesel::pg::PgConnection
// ```
// 
// To use a different table id type, change this as you want:
//
// ```
// id: &uuid::Uuid
// ```
//
// To use a database connection that does not provide `.get_results`,
// you must omit the macros named `*_with_results`; the easy ways to
// omit the macros is to comment them, or delete them from this file.
//
#[macro_export]
macro_rules! db_create {
    ($Insertable:ty) => {

        /// DB create; this uses `.execute`.
        ///
        /// Return the row change count as `usize`.
        ///
        /// # Example
        ///
        /// ```todo
        /// let connection: diesel::pg::PgConnection = establish_connection();
        /// let insertable: $Insertable = ...;
        /// let result: QueryResult<usize> = db_create(connection, insertable);
        /// let count: usize = result.unwrap();
        /// ```

        pub fn db_create(
            connection: &diesel::pg::PgConnection,
            insertable: &$Insertable
        ) -> QueryResult<usize> {
            diesel::insert_into(table)
                .values(insertable)
                .execute(connection)
        }
    }
}

#[macro_export]
macro_rules! db_create_with_result {
    ($Insertable:ty, $Queryable:ty) => {

        /// DB create; this uses `.get_results`.
        ///
        /// Return the result.
        ///
        /// # Example
        ///
        /// ```todo
        /// let connection: diesel::pg::PgConnection = establish_connection();
        /// let insertable: $Insertable = ...;
        /// let result: Result<Queryable, Error> = db_create_with_result(connection, insertable);
        /// let queryable: Queryable = result.unwrap();
        /// ```

        pub fn db_create_with_result(
            connection: &diesel::pg::PgConnection,
            insertable: &$Insertable
        ) -> Result<$Queryable, diesel::result::Error> {
            diesel::insert_into(table)
                .values(insertable)
                .get_result::<$Queryable>(connection)
        }
    }
}

#[macro_export]
macro_rules! db_creates {
    ($Insertable:ty) => {

        /// DB creates i.e. multiple values; this uses `.execute`.
        ///
        /// Return the row change count.
        ///
        /// # Example
        ///
        /// ```todo
        /// let connection: diesel::pg::PgConnection = establish_connection();
        /// let insertable: $Insertable = ...;
        /// let result: QueryResult<usize> = db_creates(connection, insertables);
        /// let count: usize = result.unwrap();
        /// ```

        pub fn db_creates(
            connection: &diesel::pg::PgConnection,
            insertables: &Vec<$Insertable>
        ) -> QueryResult<usize> {
            diesel::insert_into(table)
                .values(insertables)
                .execute(connection)
        }
    }
}

#[macro_export]
macro_rules! db_creates_with_results {
    ($Insertable:ty, $Queryable:ty) => {

        /// DB creates i.e. multiple values; this uses `.get_results`.
        ///
        /// Return the results.
        ///
        /// # Example
        ///
        /// ```todo
        /// let connection: diesel::pg::PgConnection = establish_connection();
        /// let insertables: Vec<$Insertable> = ...;
        /// let result: Result<Queryable, Error> = db_creates_with_results(connection, insertables);
        /// let results: Vec<Queryable> = result.unwrap();
        /// ```

        pub fn db_creates_with_results(
            connection: &diesel::pg::PgConnection,
            insertables: &Vec<$Insertable>
        ) -> Result<Vec<$Queryable>, diesel::result::Error> {
            diesel::insert_into(table)
                .values(insertables)
                .get_results::<$Queryable>(connection)
        }
    }
}

#[macro_export]
macro_rules! db_read {
    ($Queryable:ty) => {

        /// DB read one row by id.
        ///
        /// Return the result.
        ///
        /// # Example
        ///
        /// ```todo
        /// let connection: diesel::pg::PgConnection = establish_connection();
        /// let id: uuid::Uuid = ...;
        /// let result: Result<Queryable, Error> = db_read(connection, id);
        /// let queryable: Queryable = result.unwrap();
        /// ```

        pub fn db_read(
            connection: &diesel::pg::PgConnection,
            id: &uuid::Uuid
        ) -> Result<$Queryable, diesel::result::Error> {
            table.find(&id).first(connection)
        }
    }
}

#[macro_export]
macro_rules! db_update {
    ($Changeset:ty) => {

        /// DB update one row by id; this uses `.execute`.
        ///
        /// Return the row change count.
        ///
        /// # Example
        ///
        /// ```todo
        /// let connection: diesel::pg::PgConnection = establish_connection();
        /// let id: uuid::Uuid = ...;
        /// let changeset: $Changeset = ...;
        /// let result: QueryResult<usize> = db_update(connection, id, changeset);
        /// let count: usize = result.unwrap();
        /// ```

        pub fn db_update(
            connection: &diesel::pg::PgConnection,
            id: &uuid::Uuid,
            changeset: &$Changeset
        ) -> QueryResult<usize> {
            diesel::update(table.find(id))
            .set(changeset)
            .execute(connection)
        }
    }
}

#[macro_export]
macro_rules! db_update_with_result {
    ($Changeset:ty, $Queryable:ty) => {

        /// DB update one row by id; this uses `.get_result`
        ///
        /// Return the result.
        ///
        /// # Example
        ///
        /// ```todo
        /// let connection: diesel::pg::PgConnection = establish_connection();
        /// let id: uuid::Uuid = ...;
        /// let changeset: $Changeset = ...;
        /// let result: Result<Queryable, Error> = db_update_with_result(connection, id, changeset);
        /// let queryable: Queryable = result.unwrap();
        /// ```

        pub fn db_update_with_result(
            connection: &diesel::pg::PgConnection,
            id: &uuid::Uuid,
            changeset: &$Changeset
        ) -> Result<$Queryable, diesel::result::Error> {
            diesel::update(table.find(id))
                .set(changeset)
                .get_result::<$Queryable>(connection)
        }
    }
}

#[macro_export]
macro_rules! db_delete {
    () => {

        /// DB delete one row by id; this uses `.execute`.
        ///
        /// Return the row change count.
        ///
        /// # Example
        ///
        /// ```todo
        /// let connection: diesel::pg::PgConnection = establish_connection();
        /// let id: uuid::Uuid = ...;
        /// let result: QueryResult<usize> = db_delete(connection, id);
        /// let count: usize = result.unwrap();
        /// ```

        pub fn db_delete(
            connection: &diesel::pg::PgConnection,
            id: &uuid::Uuid
        ) -> QueryResult<usize> {
            diesel::delete(table.find(id))
                .execute(connection)
        }
    }
}

#[macro_export]
macro_rules! db_delete_with_result {
    ($Queryable:ty) => {

        /// DB delete one row by id; this uses `.get_result`.
        ///
        /// Return the result.
        ///
        /// # Example
        ///
        /// ```todo
        /// let connection: diesel::pg::PgConnection = establish_connection();
        /// let id: uuid::Uuid = ...;
        /// let result: Result<Queryable, Error> = db_create_with_result(connection, id);
        /// let queryable: Queryable = result.unwrap();
        /// ```

        pub fn db_delete_with_result(
            connection: &diesel::pg::PgConnection,
            id: &uuid::Uuid
        ) -> Result<$Queryable, diesel::result::Error> {
            diesel::delete(table.find(id))
                .get_result::<$Queryable>(connection)
        }
    }
}

#[macro_export]
macro_rules! crud {
    (
        $Queryable:ty,
        $Insertable:ty,
        $Changeset:ty,
    ) => {
        db_create!($Insertable);
        db_create_with_result!($Insertable, $Queryable);
        db_creates!($Insertable);
        db_creates_with_results!($Insertable, $Queryable);
        db_read!($Queryable);
        db_update!($Changeset);
        db_update_with_result!($Changeset, $Queryable);
        db_delete!();
        db_delete_with_result!($Queryable);
    }
}
