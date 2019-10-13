use ::diesel::{prelude::*, result::QueryResult};

use crate::schema::arcs as s;
use s::table as table;
use crate::models::arc as t;
use t::arc::Arc as T;

impl T {
    db_crud!(
        crate::models::arc::arc::Arc,
        crate::models::arc::arc::Arc,
        crate::models::arc::arc::Arc,
    );
}
