use diesel::result::Error;
use diesel::result::QueryResult;

/// For documentation, see corresponding macro `db_crud`.

pub trait DBCrud<
    Id, 
    Queryable, 
    Insertable, 
    Changesettable
> {

    fn db_create(
        insertable: &Insertable
    ) -> QueryResult<usize>;

    fn db_create_with_result(
        insertable: &Insertable
    ) -> Result<Queryable, Error>;

    fn db_creates(
        insertables: &Vec<Insertable>
    ) -> QueryResult<usize>;

    fn db_creates_with_results(
        insertables: &Vec<Insertable>
    ) -> Result<Vec<Queryable>, Error>;

    fn db_read(
        id: &Id
    ) -> Result<Queryable, Error>;

    fn db_update(
        id: &Id,
        changesettable: &Changesettable
    ) -> QueryResult<usize>;

    fn db_update_with_result(
        id: &Id,
        changesettable: &Changesettable
    ) -> Result<Queryable, Error>;

    fn db_delete(
        id: &Id
    ) -> QueryResult<usize>;

    fn db_delete_with_result(
        id: &Id
    ) -> Result<Queryable, Error>;

    fn db_count(
    ) -> QueryResult<i64>;

    fn db_all(
    ) -> Result<Vec<Queryable>, Error>;

}
