use clap::{App, Arg, ArgMatches};
use crate::types;
use crate::models::arc::arc::Arc as T;

impl T {
    
    fn arg_subject_uri<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("subject_uri")
            .help("subject URI; example: \"http://example.com/alpha/\"")
            .long("subject_uri")
            .value_name("URI")
            .takes_value(true)
    }

    fn arg_subject_database<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("subject_database")
            .help("subject database name; example: \"contactopensource\"")
            .long("subject_database")
            .value_name("DB DATABASE NAME")
            .takes_value(true)
    }

    fn arg_subject_schema<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("subject_schema")
            .help("subject schema name; example: \"public\"")
            .long("subject_schema")
            .value_name("DB SCHEMA NAME")
            .takes_value(true)
    }

    fn arg_subject_table<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("subject_table")
            .help("subject table name; example: \"persons\"")
            .long("subject_table")
            .value_name("DB TABLE NAME")
            .takes_value(true)
    }

    fn arg_subject_id<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("subject_id")
            .help("source id; example: bba6d2f6df708732f5eb2937e35b3d93")
            .long("subject_id")
            .value_name("DB ROW ID")
            .takes_value(true)
    }

    fn arg_predicate_uri<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("predicate_uri")
            .help("predicate URI; example: \"http://example.com/alpha/\"")
            .long("predicate_uri")
            .value_name("URI")
            .takes_value(true)
    }

    fn arg_predicate_database<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("predicate_database")
            .help("predicate database name; example: \"contactopensource\"")
            .long("predicate_database")
            .value_name("DB DATABASE NAME")
            .takes_value(true)
    }

    fn arg_predicate_schema<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("predicate_schema")
            .help("predicate schema name; example: \"public\"")
            .long("predicate_schema")
            .value_name("DB SCHEMA NAME")
            .takes_value(true)
    }

    fn arg_predicate_table<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("predicate_table")
            .help("predicate table name; example: \"persons\"")
            .long("predicate_table")
            .value_name("DB TABLE NAME")
            .takes_value(true)
    }

    fn arg_predicate_id<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("predicate_id")
            .help("source id; example: 8218b60194752ebfe973f68ecfcf38ea")
            .long("predicate_id")
            .value_name("DB ROW ID")
            .takes_value(true)
    }

    fn arg_object_uri<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("object_uri")
            .help("object URI; example: \"http://example.com/alpha/\"")
            .long("object_uri")
            .value_name("URI")
            .takes_value(true)
    }

    fn arg_object_database<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("object_database")
            .help("object database name; example: \"contactopensource\"")
            .long("object_database")
            .value_name("DB DATABASE NAME")
            .takes_value(true)
    }

    fn arg_object_schema<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("object_schema")
            .help("object schema name; example: \"public\"")
            .long("object_schema")
            .value_name("DB SCHEMA NAME")
            .takes_value(true)
    }

    fn arg_object_table<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("object_table")
            .help("object table name; example: \"persons\"")
            .long("object_table")
            .value_name("DB TABLE NAME")
            .takes_value(true)
    }

    fn arg_object_id<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("object_id")
            .help("source id; example: 8218b60194752ebfe973f68ecfcf38ea")
            .long("object_id")
            .value_name("DB ROW ID")
            .takes_value(true)
    }

    fn arg_start_at_timestamp_utc<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("start_at_timestamp_utc")
            .help("start at timestamp; example: \"2021-01-01T00:00:00Z\"")
            .long("start_at_timestamp_utc")
            .value_name("TIMESTAMP")
            .takes_value(true)
    }

    fn arg_stop_at_timestamp_utc<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("stop_at_timestamp_utc")
            .help("stop at timestamp; example: \"2021-01-01T00:00:00Z\"")
            .long("stop_at_timestamp_utc")
            .value_name("TIMESTAMP")
            .takes_value(true)
    }

    fn arg_count<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("count")
            .help("count, such as an instance index; example: 10 means count 10")
            .long("count")
            .value_name("NUMBER")
            .takes_value(true)
    }

    fn arg_weight<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("weight")
            .help("weight, such as a graph cost or length; example: 2 means weight 2")
            .long("weight")
            .value_name("NUMBER")
            .takes_value(true)
    }

    fn arg_probability<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("probability")
            .help("probability, 0 to 1 inclusive; example: 0.1 means probability 10%")
            .long("probability")
            .value_name("NUMBER")
            .takes_value(true)
    }

}

impl crate::traits::clap_able::ClapAble for T {

    fn init_clap_app<'a, 'ar>(app: App<'a, 'ar>) -> App<'a, 'ar> {
        let app = app.arg(Self::arg_subject_uri());
        let app = app.arg(Self::arg_subject_database());
        let app = app.arg(Self::arg_subject_schema());
        let app = app.arg(Self::arg_subject_table());
        let app = app.arg(Self::arg_subject_id());
        let app = app.arg(Self::arg_predicate_uri());
        let app = app.arg(Self::arg_predicate_database());
        let app = app.arg(Self::arg_predicate_schema());
        let app = app.arg(Self::arg_predicate_table());
        let app = app.arg(Self::arg_predicate_id());
        let app = app.arg(Self::arg_object_uri());
        let app = app.arg(Self::arg_object_database());
        let app = app.arg(Self::arg_object_schema());
        let app = app.arg(Self::arg_object_table());
        let app = app.arg(Self::arg_object_id());
        let app = app.arg(Self::arg_start_at_timestamp_utc());
        let app = app.arg(Self::arg_stop_at_timestamp_utc());
        let app = app.arg(Self::arg_count());
        let app = app.arg(Self::arg_weight());
        let app = app.arg(Self::arg_probability());
        app
    }

    fn from_clap_arg_matches(matches: &ArgMatches) -> T {
        T {
            id: types::id::from_option_str(matches.value_of("id")).unwrap(),

            // Programming-related
            tenant_id: types::id::from_option_str(matches.value_of("tenant_id")),
            typecast: types::typecast::from_option_str(matches.value_of("typecast")),
            state: types::state::from_option_str(matches.value_of("state")),

            //TODO learn
            // tenant_id: match matches.value_of("tenant") {
            //     None => None,
            //     Some(s) => Some(types::id::from_str(s).unwrap_or(None)),
            // },
            // typecast: match matches.value_of("typecast") {
            //     None => None,
            //     Some(s) => Some(types::typecast::from_str(s).unwrap_or(None)),
            // },
            // state: match matches.value_of("state") {
            //     None => None,
            //     Some(s) => Some(types::state::from_str(s).unwrap_or(None)),
            // },

            // Update-related
            updated_at_timestamp_utc: types::timestamp::from_option_str(matches.value_of("updated_at_timestamp_utc")),
            updated_at_clock_count: types::count::from_option_str(matches.value_of("updated_at_clock_count")),
            updated_by_text: types::text::from_option_str(matches.value_of("updated_by_text")),

            // Subject
            subject_uri: types::uri_string::from_option_str(matches.value_of("subject_uri")),
            subject_database: types::db_database_name::from_option_str(matches.value_of("subject_database")),
            subject_schema: types::db_schema_name::from_option_str(matches.value_of("subject_schema")),
            subject_table: types::db_table_name::from_option_str(matches.value_of("subject_table")),
            subject_id: types::db_row_id::from_option_str(matches.value_of("subject_id")),

            // Predicate
            predicate_uri: types::uri_string::from_option_str(matches.value_of("subject_uri")),
            predicate_database: types::db_database_name::from_option_str(matches.value_of("subject_database")),
            predicate_schema: types::db_schema_name::from_option_str(matches.value_of("subject_schema")),
            predicate_table: types::db_table_name::from_option_str(matches.value_of("subject_table")),
            predicate_id: types::db_row_id::from_option_str(matches.value_of("subject_id")),

            // Object
            object_uri: types::uri_string::from_option_str(matches.value_of("subject_uri")),
            object_database: types::db_database_name::from_option_str(matches.value_of("subject_database")),
            object_schema: types::db_schema_name::from_option_str(matches.value_of("subject_schema")),
            object_table: types::db_table_name::from_option_str(matches.value_of("subject_table")),
            object_id: types::db_row_id::from_option_str(matches.value_of("subject_id")),

            // Lifecycle
            start_at_timestamp_utc: types::timestamp::from_option_str(matches.value_of("start_at_timestamp_utc")),
            stop_at_timestamp_utc: types::timestamp::from_option_str(matches.value_of("stop_at_timestamp_utc")),

            // Modifiers
            count: types::count::from_option_str(matches.value_of("count")),
            weight: types::weight::from_option_str(matches.value_of("weight")),
            probability: types::probability::from_option_str(matches.value_of("probability")),
        }
    }
}
