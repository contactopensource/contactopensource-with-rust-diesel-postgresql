use ::diesel::{prelude::*, result::QueryResult};

use crate::schema::items as s;
use s::table as table;
use crate::models::item as t;
use t::item::Item as T;

// use ::diesel::PgConnection as MyConnection;
// use ::diesel::result::Error as MyError;
// use uuid::Uuid as MyId;
// use t::item::Item as MyModel;
// use t::item::Item as MyQueryable;
// use t::item::Item as MyInsertable;
// use t::item::Item as MyChangeset;

impl T {
    db_crud!(
        crate::models::item::item::Item,
        crate::models::item::item::Item,
        crate::models::item::item::Item,
    );
}
