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

pub mod schema;

pub mod traits {
    pub mod as_serde_json_value;
    pub mod as_sql_insert;
    pub mod clap_able;
    pub mod type_contains;
}

pub mod types;
pub use types as t;

pub mod models {
    pub mod item {
        pub mod item;
        pub mod as_serde_json_value;
        pub mod as_sql_insert;
        pub mod clap;
        pub mod crud;
        pub mod fab;
    }
    pub mod arc {
        pub mod arc;
        pub mod as_serde_json_value;
        pub mod as_sql_insert;
        pub mod clap;
        pub mod crud;
        pub mod fab;
    }
    pub mod org {
        pub mod org;
        pub mod as_serde_json_value;
        pub mod as_sql_insert;
        pub mod clap;
        pub mod crud;
        pub mod fab;
    }
    pub mod person {
        pub mod person;
        pub mod as_serde_json_value;
        pub mod as_sql_insert;
        pub mod clap;
        pub mod crud;
        pub mod fab;
    }
    pub mod place {
        pub mod place;
        pub mod as_serde_json_value;
        pub mod as_sql_insert;
        pub mod clap;
        pub mod crud;
        pub mod fab;
    }
    pub mod thing {
        pub mod thing;
        pub mod as_serde_json_value;
        pub mod as_sql_insert;
        pub mod clap;
        pub mod crud;
        pub mod fab;
    }
    pub mod event {
        pub mod event;
        pub mod as_serde_json_value;
        pub mod as_sql_insert;
        pub mod clap;
        pub mod crud;
        pub mod fab;
    }
    pub mod link_contact {
        pub mod link_contact;
        pub mod as_serde_json_value;
        pub mod as_sql_insert;
        pub mod clap;
        pub mod crud;
        pub mod fab;
    }
    pub mod email_contact {
        pub mod email_contact;
        pub mod as_serde_json_value;
        pub mod as_sql_insert;
        pub mod clap;
        pub mod crud;
        pub mod fab;
    }
    pub mod telephone_contact {
        pub mod telephone_contact;
        pub mod as_serde_json_value;
        pub mod as_sql_insert;
        pub mod clap;
        pub mod crud;
        pub mod fab;
    }
    pub mod passport_contact {
        pub mod passport_contact;
        pub mod as_serde_json_value;
        pub mod as_sql_insert;
        pub mod clap;
        pub mod crud;
        pub mod fab;
    }
    pub mod postal_contact {
        pub mod postal_contact;
        pub mod as_serde_json_value;
        pub mod as_sql_insert;
        pub mod clap;
        pub mod crud;
        pub mod fab;
    }
    pub mod contact {
        pub mod contact;
        pub mod as_serde_json_value;
        pub mod as_sql_insert;
        pub mod clap;
        pub mod crud;
        pub mod fab;
    }
    pub mod tag {
        pub mod tag;
        pub mod as_serde_json_value;
        pub mod as_sql_insert;
        pub mod clap;
        pub mod crud;
        pub mod fab;
    }
    pub mod code {
        pub mod code;
        pub mod as_serde_json_value;
        pub mod as_sql_insert;
        pub mod clap;
        pub mod crud;
        pub mod fab;
    }
    pub mod locale {
        pub mod locale;
        pub mod as_serde_json_value;
        pub mod as_sql_insert;
        pub mod clap;
        pub mod crud;
        pub mod fab;
    }
    pub mod media_type {
        pub mod media_type;
        pub mod as_serde_json_value;
        pub mod as_sql_insert;
        pub mod clap;
        pub mod crud;
        pub mod fab;
    }
    pub mod geo_point {
        pub mod geo_point;
        pub mod as_serde_json_value;
        pub mod as_sql_insert;
        pub mod clap;
        pub mod crud;
        pub mod fab;
    }
    pub mod geo_code {
        pub mod geo_code;
        pub mod as_serde_json_value;
        pub mod as_sql_insert;
        pub mod clap;
        pub mod crud;
        pub mod fab;
    }
    pub mod person_name {
        pub mod person_name;
        pub mod as_serde_json_value;
        pub mod as_sql_insert;
        pub mod clap;
        pub mod crud;
        pub mod fab;
    }
    pub mod person_pronoun {
        pub mod person_pronoun;
        pub mod as_serde_json_value;
        pub mod as_sql_insert;
        pub mod clap;
        pub mod crud;
        pub mod fab;
    }
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
