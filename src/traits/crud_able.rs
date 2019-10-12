//TODO: This file is WIP and intentionally commented out.

// use diesel::{prelude::*, pg::PgConnection as Connection, result::QueryResult};

//TODO metaprogram id to be any supportable type e.g. diesel::associations::Identifiable
// use crate::types::id::Id;

pub trait CrudAble {

    //// DB create; this uses `.execute`.
    ////
    //// Return the row change count as `usize`.
    ////
    //// # Example
    ////
    //// ```
    //// let result: QueryResult<usize> = db_create(table, insertable);
    //// let count: usize = result.unwrap();
    //// ```

    // fn db_create<T: Table, U: diesel::prelude::Insertable>(
    //     connection: &Connection,
    //     table: &T,
    //     insertable: &U
    // ) -> QueryResult<usize> {
    //     diesel::insert_into(table)
    //         .values(insertable)
    //         .execute(connection)
    // }

    //// DB create; this uses `.get_results`.
    ////
    //// Return the result.
    ////
    //// # Example
    ////
    //// ```
    //// let result: Result<Queryable, Error> = db_create_with_result(table, insertable);
    //// let queryable: Queryable = result.unwrap();
    //// ```

    // fn db_create_with_result<T: Table, U: diesel::prelude::Insertable, Q: diesel::prelude::Queryable>(
    //     connection: &Connection,
    //     table: &T,
    //     insertable: &U
    // ) -> Result<Q, diesel::result::Error> {
    //     diesel::insert_into(table)
    //         .values(insertable)
    //         .get_result::<Q>(connection)
    // }

    //// DB creates i.e. multiple values; this uses `.execute`.
    ////
    //// Return the row change count.
    ////
    //// # Example
    ////
    //// ```
    //// let result: QueryResult<usize> = db_creates(table, insertables);
    //// let count: usize = result.unwrap();
    //// ```

    // fn db_creates<T: Table, U: diesel::prelude::Insertable>(
    //     connection: &Connection,
    //     table: &T,
    //     insertables: &Vec<U>
    // ) -> QueryResult<usize> {
    //     diesel::insert_into(table)
    //         .values(insertables)
    //         .execute(connection)
    // }

    //// DB creates i.e. multiple values; this uses `.get_results`.
    ////
    //// Return the results.
    ////
    //// # Example
    ////
    //// ```
    //// let result: Result<Queryable, Error> = db_creates_with_results(table, insertables);
    //// let results: Vec<Queryable> = result.unwrap();
    //// ```
    ////

    // fn db_creates_with_results<T: Table, U: diesel::prelude::Insertable, Q: diesel::prelude::Queryable>(
    //     connection: &Connection,
    //     table: &T,
    //     insertables: &Vec<U>
    // ) -> Result<Vec<Q>, diesel::result::Error> {
    //     diesel::insert_into(table)
    //         .values(insertables)
    //         .get_results::<T>(connection)
    // }

    //// DB read one row by id.
    ////
    //// Return the result.
    ////
    //// # Example
    ////
    //// ```
    //// let result: Result<Queryable, Error> = db_read(table, id);
    //// let queryable: Queryable = result.unwrap();
    //// ```

    // fn db_read<T: Table, Q: diesel::prelude::Queryable>(
    //     connection: &Connection,
    //     table: &T,
    //     id: &Id
    // ) -> Result<Q, diesel::result::Error> {
    //     table.find(&id).first(connection)
    // }

    //// DB update one row; this uses `.execute`.
    ////
    //// Return the row change count.
    ////
    //// # Example
    ////
    //// ```
    //// let result: QueryResult<usize> = db_update(connection, table, id, changeset);
    //// let count: usize = result.unwrap();
    //// ```
    ////

    // fn db_update<T: Table, U: diesel::query_builder::AsChangeset>(
    //     connection: &Connection,
    //     table: &T,
    //     id: &Id,
    //     changeset: &U
    // ) -> QueryResult<usize> {
    //     diesel::update(table.find(id))
    //     .set(changeset)
    //     .execute(connection)
    // }

    //// DB update one row; this uses `.get_result`
    ////
    //// Return the result as a `AsChangeset`.
    ////
    //// # Example
    ////
    //// ```
    //// let result: Result<Queryable, diesel::result::Error> = db_update_with_result(connection, table, id, changeset);
    //// let queryable: Queryable = result.unwrap();
    //// ```

    // fn db_update_with_result<T: Table, U: diesel::query_builder::AsChangeset, Q: diesel::prelude::Queryable>(
    //     connection: &Connection,
    //     table: &T,
    //     id: &Id,
    //     changeset: &U
    // ) -> Result<Q, diesel::result::Error> {
    //     diesel::update(table.find(id))
    //         .set(changeset)
    //         .get_result::<T>(connection)
    // }

    //// DB delete one row by id; this uses `.execute`.
    ////
    //// Return the row change count.
    ////
    //// # Example
    ////
    //// ```
    //// let result: QueryResult<usize> = db_delete(table, id);
    //// let count: usize = result.unwrap();
    //// ```

    // fn db_delete<T: Table>(
    //     connection: &Connection,
    //     table: &T,
    //     id: &Id
    // ) -> QueryResult<usize> {
    //     diesel::delete(table.find(id))
    //         .execute(connection)
    // }

    //// DB delete one row by id; this uses `.get_results`.
    //// Returns result model.
    //// # Example
    ////
    //// ```
    //// let result: Result<Queryable, Error> = db_delete_with_result(table, id);
    //// let queryable: Queryable = result.unwrap();
    //// ```

    // fn db_delete_with_result<T: Table, Q: diesel::prelude::Queryable>(
    //     connection: &Connection,
    //     table: &T,
    //     id: &Id
    // ) -> Result<Q, diesel::result::Error> {
    //     diesel::delete(table.find(id))
    //         .get_result::<T>(connection)
    // }

}
