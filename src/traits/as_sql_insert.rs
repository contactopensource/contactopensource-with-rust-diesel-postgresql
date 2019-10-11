pub trait AsSqlInsert {
    fn as_sql_insert(&self) -> String;
}
