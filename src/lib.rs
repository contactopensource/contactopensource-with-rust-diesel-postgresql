#![recursion_limit="1024"]

// bigdecimal: very large decimal numbers with exact precision
pub use ::bigdecimal;

// chrono:: date and time
pub use ::chrono;
//use ::chrono::prelude::*;
//use ::chrono::Duration;

// diesel: database ORM
#[macro_use]
extern crate diesel;
use ::diesel::{prelude::*, pg::PgConnection};

// dotenv: environment variables
extern crate dotenv;
use ::dotenv::dotenv;

// itertools: iterator tools
//#[macro_use]
//extern crate itertools;
//use ::itertools::Itertools;

// lazy_static: a macro for declaring lazily evaluated statics in Rust.
#[macro_use]
extern crate lazy_static;

// Num is for numbers, including BigInt, and features for rand and Serde.
extern crate num;

//use r2d2;

// rand: random numbers, choosers, distributions, etc.
pub use ::rand::{self, Rng, thread_rng, seq::SliceRandom};

// Serde: Serialize and Deserialize, such as by Diesel models.
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate time;  // TODO remove the `time` crate when Chrono updates to use standard Rust time

//use ::std::error::Error;
//use ::std::time::SystemTime;
use ::std::env;

// uuid: Universal Unique Identifier
extern crate uuid;
pub use ::uuid::Uuid;

#[macro_use] 
pub mod macros {
    #[macro_use] 
    pub mod db_crud;
}

pub mod schema;

pub mod traits {
    pub mod as_serde_json_value;
    pub mod as_sql_insert;
    pub mod clap_able;
    pub mod type_contains;
}

pub mod types;
pub use types as t;

macro_rules! model {
    ( $name:ident ) => {
        pub mod $name {
            pub mod $name;
            pub mod as_serde_json_value;
            pub mod as_sql_insert;
            pub mod clap;
            pub mod db_crud;
            pub mod fab;
        }
    };
}

pub mod models {
    model!(item);
    model!(arc);
    model!(org);
    model!(person);
    model!(place);
    model!(thing);
    model!(event);
    model!(link_contact);
    model!(email_contact);
    model!(telephone_contact);
    model!(passport_contact);
    model!(postal_contact);
    model!(contact);
    model!(tag);
    model!(code);
    model!(locale);
    model!(media_type);
    model!(geo_point);
    model!(geo_code);
    model!(person_name);
    model!(person_pronoun);
}

pub mod helpers {
    pub mod parse {
        pub mod a;
        pub mod big_decimal;
        pub mod chrono;
        pub mod serde_json;
        pub mod uuid;
    }
    pub mod fab {
        pub mod chrono;
    }
    pub mod zid;
}

//use crate::types;
//use crate::models as m;
//use crate::traits::*;

// use crate::traits::as_serde_json_value::AsSerdeJsonValue;
// use crate::traits::as_sql_insert::AsSqlInsert;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
