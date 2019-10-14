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
// # Dependencies
//
// These macros are specific to our app's usage:
//
//   * The crate `diesel` for object-relational mapping.
//
//   * The PostgreSQL database.
//
//   * The Diesel PostgreSQL connection type.
//
//   * The Diesel PostgreSQL capability to call `.get_results`.
//
//   * The crate `uuid` for our table primary key type UUID.
//
//   * The function `crate::db_connection` which returns a connection.
//
// See below for customization.
//
// # Usage
//
// Write a typical Diesel model, such as a model named `Item`,
// and similar `ItemInsertable`, `ItemChangesettable`.
//
// This macro needs these Diesel traits:
//
//   * Queryaable
//
//   * Insertable
//
//   * AsChangeset
//
// ```
// TODO fix
//
// impl crate::traits::db_crud::DBCrud<
//     uuid::Uuid,
//     crate::models::item::item::Item,
//     crate::models::item::item::ItemInsertable,
//     crate::models::item::item::ItemChangesettable,
// > for crate::models::item::item::Item {
//     db_crud!(
//        crate::db_connection,
//        crate::schema::item::table,
//        uuid::Uuid,
//        crate::models::item::item::Item,
//        crate::models::item::item::ItemInsertable,
//        crate::models::item::item::ItemChangesettable,
//    );
// }
//}
//
// # Customization
//
// You can configure these macros for your own app.
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

#[macro_export]
macro_rules! db_crud {
    (        
        $Connection:expr,
        $Table:expr,
        $Id:ty,
        $Queryable:ty,
        $Insertable:ty,
        $Changesettable:ty,
    ) => {

        /// DB create; this uses `.execute`.
        ///
        /// Return the row change count as `usize`.
        ///
        /// # Example
        ///
        /// ```todo
        /// let insertable: $Insertable = ...;
        /// let result: QueryResult<usize> = db_create(insertable);
        /// let count: usize = result.unwrap();
        /// ```

        #[allow(dead_code)]
        fn db_create(
            insertable: &$Insertable
        ) -> QueryResult<usize> {
            let connection = $Connection();
            diesel::insert_into($Table)
                .values(insertable)
                .execute(&connection)
        }

        /// DB create; this uses `.get_results`.
        ///
        /// Return the result.
        ///
        /// # Example
        ///
        /// ```todo
        /// let insertable: $Insertable = ...;
        /// let result: Result<Queryable, Error> = db_create_with_result(insertable);
        /// let row: Queryable = result.unwrap();
        /// ```

        #[allow(dead_code)]
        fn db_create_with_result(
            insertable: &$Insertable
        ) -> Result<$Queryable, diesel::result::Error> {
            let connection = $Connection();
            diesel::insert_into($Table)
                .values(insertable)
                .get_result::<$Queryable>(&connection)
        }

        /// DB creates i.e. multiple values; this uses `.execute`.
        ///
        /// Return the row change count.
        ///
        /// # Example
        ///
        /// ```todo
        /// let insertable: Vec<$Insertable = ...;
        /// let result: QueryResult<usize> = db_creates(insertables);
        /// let count: usize = result.unwrap();
        /// ```

        #[allow(dead_code)]
        fn db_creates(
            insertables: &Vec<$Insertable>
        ) -> QueryResult<usize> {
            let connection = $Connection();
            diesel::insert_into($Table)
                .values(insertables)
                .execute(&connection)
        }

        /// DB creates i.e. multiple values; this uses `.get_results`.
        ///
        /// Return the results.
        ///
        /// # Example
        ///
        /// ```todo
        /// let insertables: Vec<$Insertable> = ...;
        /// let result: Result<Queryable, Error> = db_creates_with_results(connection, insertables);
        /// let rows: Vec<Queryable> = result.unwrap();
        /// ```

        #[allow(dead_code)]
        fn db_creates_with_results(
            insertables: &Vec<$Insertable>  
        ) -> Result<Vec<$Queryable>, diesel::result::Error> {
            let connection = $Connection();
            diesel::insert_into($Table)
                .values(insertables)
                .get_results::<$Queryable>(&connection)
        }

        /// DB read one row by id.
        ///
        /// Return the result.
        ///
        /// # Example
        ///
        /// ```todo
        /// let id: Id = ...;
        /// let result: Result<Queryable, Error> = db_read(id);
        /// let row: Queryable = result.unwrap();
        /// ```

        #[allow(dead_code)]
        fn db_read(
            id: &$Id
        ) -> Result<$Queryable, diesel::result::Error> {
            let connection = $Connection();
            $Table.find(&id).first(&connection)
        }

        /// DB update one row by id; this uses `.execute`.
        ///
        /// Return the row change count.
        ///
        /// # Example
        ///
        /// ```todo
        /// let id: Id = ...;
        /// let changesettable: $Changesettable= ...;
        /// let result: QueryResult<usize> = db_update(id, changeset);
        /// let count: usize = result.unwrap();
        /// ```

        #[allow(dead_code)]
        fn db_update(
            id: &$Id,
            changesettable: &$Changesettable
        ) -> QueryResult<usize> {
            let connection = $Connection();
            diesel::update($Table.find(id))
            .set(changesettable)
            .execute(&connection)
        }

        /// DB update one row by id; this uses `.get_result`
        ///
        /// Return the result.
        ///
        /// # Example
        ///
        /// ```todo
        /// let id: Id = ...;
        /// let changesettable: $Changesettable= ...;
        /// let result: Result<Queryable, Error> = db_update_with_result(id, changeset);
        /// let row: Queryable = result.unwrap();
        /// ```

        #[allow(dead_code)]
        fn db_update_with_result(
            id: &$Id,
            changesettable: &$Changesettable
        ) -> Result<$Queryable, diesel::result::Error> {
            let connection = $Connection();
            diesel::update($Table.find(id))
                .set(changesettable)
                .get_result::<$Queryable>(&connection)
        }

        /// DB delete one row by id; this uses `.execute`.
        ///
        /// Return the row change count.
        ///
        /// # Example
        ///
        /// ```todo
        /// let id: Id = ...;
        /// let result: QueryResult<usize> = db_delete(id);
        /// let count: usize = result.unwrap();
        /// ```

        #[allow(dead_code)]
        fn db_delete(
            id: &$Id
        ) -> QueryResult<usize> {
            let connection = $Connection();
            diesel::delete($Table.find(id))
                .execute(&connection)
        }

        /// DB delete one row by id; this uses `.get_result`.
        ///
        /// Return the result.
        ///
        /// # Example
        ///
        /// ```todo
        /// let id: Id = ...;
        /// let result: Result<Queryable, Error> = db_create_with_result(id);
        /// let row: Queryable = result.unwrap();
        /// ```

        #[allow(dead_code)]
        fn db_delete_with_result(
            id: &$Id
        ) -> Result<$Queryable, diesel::result::Error> {
            let connection = $Connection();
            diesel::delete($Table.find(id))
                .get_result::<$Queryable>(&connection)
        }

        /// DB count of rows.
        ///
        /// Return the count.
        ///
        /// # Example
        ///
        /// ```todo
        /// let result: QueryResult<i64> = db_count();
        /// let count: i64 = result.unwrap();
        /// ```

        #[allow(dead_code)]
        fn db_count(
        ) -> QueryResult<i64> {
            let connection = $Connection();
            $Table.count().get_result(&connection)
        }

        /// DB all rows.
        ///
        /// Return all the table's rows.
        ///
        /// # Example
        ///
        /// ```todo
        /// let result: QueryResult<i64> = db_all();
        /// let rows: Vec<Queryable> = result.unwrap();
        /// ```

        #[allow(dead_code)]
        fn db_all(
        ) -> Result<Vec<$Queryable>, diesel::result::Error> {
            let connection = $Connection();
            $Table.load::<$Queryable>(&connection)
        }

    }
}
