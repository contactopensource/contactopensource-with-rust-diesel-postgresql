#![recursion_limit="1024"]

extern crate clap;
extern crate diesel;
extern crate diesel_dynamic_schema;
extern crate bigdecimal;
extern crate chrono;
extern crate dotenv;
extern crate r2d2;
extern crate rand;
extern crate uuid;

use ::diesel::prelude::*;
//use ::diesel_dynamic_schema::{schema, table}; //TODO use

extern crate contactopensource;
use ::contactopensource::{schema}; //TODO add models, traits
//use ::contactopensource::helpers::parse;
use ::contactopensource::traits::as_serde_json_value::AsSerdeJsonValue;
//use ::contactopensource::traits::as_sql_insert::AsSqlInsert;
use ::contactopensource::models::{contact::contact::Contact};

use clap::{App, Arg, ArgMatches, SubCommand};
use dotenv::dotenv;
//use ::uuid::Uuid;
use std::env;

//#[macro_use] extern crate maplit;

enum OutputFormat {
    Text,
    JSON,
    HTML,
    XML,
}

struct Config {
    verbose: bool,
    output_format: OutputFormat,
}

////
//
// App: create the clap app and all its settings
//
////

fn app_with_verbose<'a, 'ar>(app: App<'a, 'ar>) -> App<'a, 'ar> {
    app
    .arg(Arg::with_name("verbose")
        .help("verbose output")
        .short("v")
        .long("verbose"))
}

fn app_with_output_format<'a, 'ar>(app: App<'a, 'ar>) -> App<'a, 'ar> {
    app
    .arg(Arg::with_name("output-text")
        .help("output format is text")
        .long("output-text"))
    .arg(Arg::with_name("output-json")
        .help("output format is JSON")
        .long("output-json"))
    .arg(Arg::with_name("output-html")
        .help("output format is HTML")
        .long("output-html"))
    .arg(Arg::with_name("output-xml")
        .help("output format is XML")
        .long("output-xml"))
}

fn app_with_common_fields<'a, 'ar>(app: App<'a, 'ar>) -> App<'a, 'ar> {
    app
    .arg(Arg::with_name("id")
        .help("id as a 32-character lowercase hexadecimal; example: 4d8453c1d45b045c6716699326c7b7fb")
        .long("id")
        .value_name("ZID")
        .takes_value(true))
    .arg(Arg::with_name("tenant_id")
        .help("tenant of the data; example: 7bd380209cd310d3ad4e7f980298cbea")
        .long("tenant_id")
        .value_name("ZID")
        .takes_value(true))
    .arg(Arg::with_name("typecast")
        .help("typecast of the data such as for single table inheritance; example: \"friend\"")
        .long("typecast")
        .value_name("TEXT")
        .takes_value(true))
    .arg(Arg::with_name("state")
        .help("state of the data such as for a state machine; example: \"active\"")
        .long("typecast")
        .value_name("TEXT")
        .takes_value(true))
    .arg(Arg::with_name("updated_at_timestamp_utc")
        .help("updated at timestamp UTC; example: \"2021-01-01T00:00:00Z\"")
        .long("updated_at_timestamp_utc")
        .value_name("TIMESTAMP")
        .takes_value(true))
    .arg(Arg::with_name("updated_at_clock_count")
        .help("updated at clock counter; example: \"123456789\"")
        .long("updated_at_clock_count")
        .value_name("COUNTER")
        .takes_value(true))
    .arg(Arg::with_name("updated_by_text")
        .help("updated by, in other words, typically freeform; example: \"alice.anderson@example.com\"")
        .long("updated_by_text")
        .value_name("TEXT")
        .takes_value(true))
}

fn app_arg_value_added_tax_identification_number<'a, 'ar>(app: App<'a, 'ar>) -> App<'a, 'ar> {
    app.arg(Arg::with_name("value_added_tax_identification_number")
        .help("value added tax identification number; see: https://schema.org/vatID https://en.wikipedia.org/wiki/VAT_identification_number")
        .long("value_added_tax_identification_number")
        .value_name("TEXT")
        .takes_value(true))
}

fn app_arg_legal_entity_identifier<'a, 'ar>(app: App<'a, 'ar>) -> App<'a, 'ar> {
    app.arg(Arg::with_name("legal_entity_identifier")
        .help("legal entity identifier; see: https://schema.org/leiCode https://en.wikipedia.org/wiki/Legal_Entity_Identifier")
        .long("legal_entity_identifier")
        .value_name("TEXT")
        .takes_value(true))
}

fn app_arg_ticker_symbol<'a, 'ar>(app: App<'a, 'ar>) -> App<'a, 'ar> {
    app.arg(Arg::with_name("ticker_symbol")
        .help("ticker symbol; see: https://en.wikipedia.org/wiki/Ticker_symbol")
        .long("ticker_symbol")
        .value_name("TEXT")
        .takes_value(true))
}

fn app_arg_international_standard_of_industrial_classification_v4<'a, 'ar>(app: App<'a, 'ar>) -> App<'a, 'ar> {
    app.arg(Arg::with_name("international_standard_of_industrial_classification_v4")
        .help("international standard of industrial classification, revision version 4; see: https://schema.org/isicV4 https://en.wikipedia.org/wiki/International_Standard_Industrial_Classification")
        .long("international_standard_of_industrial_classification_v4")
        .value_name("TEXT")
        .takes_value(true))
}

fn app_with_subcommand_count<'a, 'ar>(app: App<'a, 'ar>) -> App<'a, 'ar> {
    app
    .subcommand(
        SubCommand::with_name("count")
        .about("Count")
        .arg(Arg::with_name("objects")
            .help("Count which objects? Example: persons")
            .takes_value(true)
            .value_name("OBJECTS")
            .required(true)
            .index(1))
    )
}

fn app_with_subcommand_list<'a, 'ar>(app: App<'a, 'ar>) -> App<'a, 'ar> {
    app
    .subcommand(
        SubCommand::with_name("list")
        .about("List")
        .arg(Arg::with_name("objects")
        .help("Count which objects? Example: persons")
        .takes_value(true)
        .value_name("OBJECTS")
            .required(true)
            .index(1))
    )
}

fn app_with_subcommand_create<'a, 'ar>(app: App<'a, 'ar>) -> App<'a, 'ar> {
    app
    .subcommand(
        SubCommand::with_name("create")
        .about("Create")
        .arg(Arg::with_name("object")
            .help("Create which object? Example: person")
            .takes_value(true)
            .value_name("OBJECT")
            .required(true)
            .index(1))
        .arg(Arg::with_name("id")
            .help("id as a 32-character lowercase hexadecimal; example: 4d8453c1d45b045c6716699326c7b7fb")
            .takes_value(true)
            .value_name("ID")
            .required(true)
            .index(2))
    )
}

fn app_with_subcommand_read<'a, 'ar>(app: App<'a, 'ar>) -> App<'a, 'ar> {
    app
    .subcommand(
        SubCommand::with_name("read")
        .about("Read")
        .arg(Arg::with_name("object")
            .help("Read which object? Example: person")
            .takes_value(true)
            .value_name("OBJECT")
            .required(true)
            .index(1))
        .arg(Arg::with_name("id")
            .help("id as a 32-character lowercase hexadecimal; example: 4d8453c1d45b045c6716699326c7b7fb")
            .takes_value(true)
            .value_name("ID")
            .required(true)
            .index(2))
    )
}

pub fn app_with_subcommand_update<'a, 'ar>(app: App<'a, 'ar>) -> App<'a, 'ar> {
    let app = app
    .subcommand(
        SubCommand::with_name("update")
        .about("Update")
        .arg(Arg::with_name("object")
            .help("Update which object? Example: person")
            .takes_value(true)
            .value_name("OBJECT")
            .required(true)
            .index(1))
        .arg(Arg::with_name("id")
            .help("id as a 32-character lowercase hexadecimal; example: 4d8453c1d45b045c6716699326c7b7fb")
            .takes_value(true)
            .value_name("ID")
            .required(true)
            .index(2))
        );
    //TODO learn how to chain app changes
    //let app = app_with_common_fields(app);
    app
}

pub fn app_with_subcommand_delete<'a, 'ar>(app: App<'a, 'ar>) -> App<'a, 'ar> {
    app
    .subcommand(
        SubCommand::with_name("delete")
        .about("Delete")
        .arg(Arg::with_name("object")
            .help("Show which object? Example: person")
            .takes_value(true)
            .value_name("OBJECT")
            .required(true)
            .index(1))
        .arg(Arg::with_name("id")
            .help("id as a 32-character lowercase hexadecimal; example: 4d8453c1d45b045c6716699326c7b7fb")
            .takes_value(true)
            .value_name("ID")
            .required(true)
            .index(2))
    )
}

pub fn app_with_subcommand_db<'a, 'ar>(app: App<'a, 'ar>) -> App<'a, 'ar> {
    app
    .subcommand(
        SubCommand::with_name("db")
        .about("Database diagnotics")
        .help("Show database diagnostics, such as table names")
    )
}

pub fn app_with_subcommand_debug<'a, 'ar>(app: App<'a, 'ar>) -> App<'a, 'ar> {
    app
    .subcommand(
        SubCommand::with_name("debug")
        .about("Debug")
        .help("Debug the application by showing diagnostics")
    )
}

pub fn app_with_subcommand_sql<'a, 'ar>(app: App<'a, 'ar>) -> App<'a, 'ar> {
    app
    .subcommand(
        SubCommand::with_name("sql")
        .about("SQL command")
        .help("Run an arbitrary SQL command string -- use with caution")
    )
}

////
//
// Run subcommands
//
////

fn run_subcommand_count(config: &Config, matches: &ArgMatches) {
    if config.verbose { output_subcommand("count") }
    let arg_action: &str = "count";
    let arg_objects: &str = matches.value_of("objects").unwrap();
    if config.verbose { output_action_objects(arg_action, arg_objects); }

    if config.verbose { output_connect(); }
    let connection = establish_connection();

    if config.verbose { output_execute(); }
    let xx = schema::contacts::table
        .load::<Contact>(&connection)
        .unwrap_or_else(|_|
            panic!("cannot {} {}", arg_action, arg_objects)
        );
    println!("count:{}", xx.len());
}

fn run_subcommand_list(config: &Config, matches: &ArgMatches) {
    if config.verbose { output_subcommand("list") }
    let arg_action: &str = "list";
    let arg_objects: &str = matches.value_of("objects").unwrap();
    if config.verbose { output_action_objects(arg_action, arg_objects); }

    if config.verbose { output_connect(); };
    let connection = establish_connection();

    if config.verbose { output_execute(); }
    let xx: Vec<Contact> = schema::contacts::table
        .load::<Contact>(&connection)
        .unwrap_or_else(|_|
            panic!("cannot {} {}", arg_action, arg_objects)
        );
    for x in xx { output_x(config, x) }
}

// fn run_subcommand_create(config: &Config, matches: &ArgMatches) {
//     if config.verbose { output_subcommand("create") }
//     let arg_action: &str = "create";
//     let arg_objects: &str = matches.value_of("objects").unwrap();
//     if config.verbose { output_action_objects(arg_action, arg_objects); }

//     if config.verbose { output_connect(); }
//     let connection = establish_connection();

//     if config.verbose { output_execute(); }

//     let x = diesel::insert_into(schema::contacts::table)
//         .values(&new_post)
//         .get_result(&connection)
//         .unwrap_or_else(|_|
//             panic!("cannot {} {}", arg_action, arg_objects)
//         );
//     output_x(&config, x);
// }

// fn run_subcommand_read(config: &Config, matches: &ArgMatches) {
//     if config.verbose { output_subcommand("read") }
//     let arg_action: &str = "read";
//     let arg_object: &str = matches.value_of("object").unwrap();
//     let arg_id: Uuid = Uuid::parse_str(matches.value_of("id").unwrap()).unwrap();
//     if config.verbose { output_action_object_id(arg_action, arg_object, &arg_id); }

//     if config.verbose { output_connect(); }
//     let connection = establish_connection();

//     if config.verbose { output_execute(); }
//     let x = schema::contacts::table.find(arg_id).first::<Contact>(&connection)
//         .unwrap_or_else(|_|
//             panic!("cannot {} {} {}", arg_action, arg_object, arg_id)
//         );

//     //TODO refactor `find`:
//     // let _pattern = format!("%{}%", matches.value_of("pattern").unwrap());
//     // let xx = contacts
//     //     .filter(name.like(_pattern))
//     //     .load::<Contact>(&connection)
//     //     .expect("find");

//     output_x(&config, x);
// }

// fn run_subcommand_update(config: &Config, matches: &ArgMatches) {
//     if config.verbose { output_subcommand("edit") }
//     let arg_action: &str = "edit";
//     let arg_object: &str = matches.value_of("object").unwrap();
//     let arg_id: Uuid = Uuid::parse_str(matches.value_of("id").unwrap()).unwrap();
//     if config.verbose { output_action_object_id(arg_action, arg_object, &arg_id); }

//     if config.verbose { output_connect(); }
//     let connection = establish_connection();

//     if config.verbose { output_execute(); }
//     let x = schema::contacts::table.find(arg_id).first::<Contact>(&connection)
//         .unwrap_or_else(|_|
//             panic!("cannot {} {} {}", arg_action, arg_object, arg_id)
//         );

//     //TODO: refactor `update`:
//     // let x = diesel::update(contacts.find(id))
//     //     .set(_parsed)
//     //     .get_result::<Contact>(&connection)
//     //     .expect(&format!("Cannot {} {} {}", action, model, id));

//     //TODO: refactor `add`:
//     // let contact = diesel::insert_into(contacts::table)
//     //     .values(&_parsed)
//     //     .get_result::<Contact>(&connection)
//     //     .expect(&format!("Cannot add id:{}", _id));


//     output_x(&config, x);
// }

// fn run_subcommand_delete(config: &Config, matches: &ArgMatches) {
//     if config.verbose { output_subcommand("delete") }
//     let arg_action: &str = "delete";
//     let arg_object: &str = matches.value_of("object").unwrap();
//     let arg_id: Uuid = Uuid::parse_str(matches.value_of("id").unwrap()).unwrap();
//     if config.verbose { output_action_object_id(arg_action, arg_object, &arg_id); }

//     if config.verbose { output_connect(); }
//     let connection = establish_connection();

//     if config.verbose { output_execute(); }
//     let x = diesel::delete(schema::contacts::table.filter(schema::contacts::id.eq(arg_id)))
//         .get_result::<Contact>(&connection)
//         .unwrap_or_else(|_|
//             panic!("Cannot {} {} {}", arg_action, arg_object, arg_id)
//         );
//     if config.verbose { output_x(&config, x) }
// }

fn run_subcommand_db(config: &Config, _matches: &ArgMatches) {
    if config.verbose { output_subcommand("db") }
    println!("db"); // TODO replace with anything more useful

    // let my_table = diesel_dynamic_schema::table("contacts");
    // println!("table:{}", my_table);

    // let my_column= my_table.column::<pg::types::sql_types::Uuid, _>("id");
    // let my_column= my_table.column<pg::types::sql_types::Uuid, _>("id");
    // println!("column:{}", my_column);

    if config.verbose { output_connect(); }
    let _connection = establish_connection();
    //TODO
    // let tablenames: Vec<String> = diesel::sql_query("SELECT tablename FROM pg_catalog.pg_tables where schemaname = 'public' and tablename not like '\_\_%' ;").load::<(String)>(&conn);
    // println!("{:?}", my_table.select((my_column)).first(&connection))
}

fn run_subcommand_debug(config: &Config, _matches: &ArgMatches) {
    if config.verbose { output_subcommand("debug") }
    println!("debug"); // TODO replace with anything more useful

    println!("primary_key:{:?}", schema::contacts::table.primary_key());

    let all_columns: <schema::contacts::table as Table>::AllColumns = schema::contacts::table::all_columns();
    println!("all_columns 0..5 {:?} {:?} {:?} {:?} {:?} {:?}",
    all_columns.0,
    all_columns.1,
    all_columns.2,
    all_columns.3,
    all_columns.4,
    all_columns.5,
    );

    // let ac: schema::contacts::SqlType = schema::contacts::table::all_columns();
    // println!("ac {:?} {:?} {:?}",
    // ac.0,
    // ac.1,
    // ac.2,
    // );

    println!("star:{:?}", schema::contacts::table.star());
    println!("star:{:?}", schema::contacts::columns::star);
}

fn run_subcommand_sql(config: &Config, _matches: &ArgMatches) {
    if config.verbose { output_subcommand("sql") }
    println!("sql"); // TODO replace with anything more useful

    if config.verbose { output_connect(); }
    let _connection = establish_connection();
    //TODO
    // results = diesel::sql_query("SELECT * FROM contacts ORDER BY id")
    // .load(&connection);

    // println!("{:?}", results)
}


////
//
// Helpers
//
////

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

////
//
// Helpers: parse input settings a.k.a. "pet" instead of "let", "get", "set".
//
////

fn pet_verbose(matches: &ArgMatches) -> bool {
    matches.is_present("verbose")
}

fn pet_output_format(matches: &ArgMatches) -> OutputFormat {
    if matches.is_present("output-text") {
        OutputFormat::Text
    }
    else if matches.is_present("output-html") {
        OutputFormat::HTML
    }
    else if matches.is_present("output-json") {
        OutputFormat::JSON
    }
    else if matches.is_present("output-xml") {
        OutputFormat::XML
    }
    else {
        OutputFormat::Text
    }
}

// pub fn table_name_to_table(table_name: &str) -> Table<ST=SqlType> {
//     match table_name {
//         "contacts" => schema::contacts::table,
//         "persons" => schema::persons::table,
//         _ => panic!("Cannot match on table name:{}", table_name),
//     }
// }

////
//
// Helpers: output
//
////

//TODO mark this function as available even though never used
// fn output(s: &str){
//     println!("{}", s);
// }

fn output_subcommand(s: &str){
    println!("subcommand:{}", s);
}

fn output_connect(){
    println!("connect...");
}

fn output_execute(){
    println!("execute...");
}

// fn output_action(_action: &str){
//     println!("action:{}", _action);
// }

// fn output_object(_object: &str){
//     println!("object:{}", _object);
// }

// fn output_id(_id: &Uuid) {
//     println!("id:{}", _id.simple())
// }

fn output_action_objects(arg_action: &str, arg_objects: &str) {
    println!("action:{} objects:{}", arg_action, arg_objects)
}

// fn output_action_object_id(arg_action: &str, arg_object: &str, arg_id: &Uuid) {
//     println!("action:{} object:{} id:{}", arg_action, arg_object, arg_id)
// }

// fn output_parsed(_parsed: &Contact) {
//     println!("parsed:{:?}", _parsed)
// }

fn output_x<T: std::fmt::Debug + AsSerdeJsonValue>(config: &Config, x: T) {
    match config.output_format {
        OutputFormat::Text => {
            println!("{:?}", x)
        },
        OutputFormat::JSON => {
            println!("{:?}", x.as_serde_json_value().to_string())
        },
        OutputFormat::HTML => {
            panic!("TODO")
        },
        OutputFormat::XML => {
            panic!("TODO")
        },
    }
}

fn main() {
    let app = App::new("ContactOpenSource")
        .version("1.0.0")
        .author("Joel Parker Henderson <joel@contactopensource.com>")
        .about("ContactOpenSource is contact relationship manager address book, open source, and free.");
    let app = app_with_verbose(app);
    let app = app_with_output_format(app);
    let app = app_with_common_fields(app);
    let app = app_with_subcommand_count(app);
    let app = app_with_subcommand_list(app);
    let app = app_with_subcommand_create(app);
    let app = app_with_subcommand_read(app);
    let app = app_with_subcommand_update(app);
    let app = app_with_subcommand_delete(app);
    let app = app_with_subcommand_db(app);
    let app = app_with_subcommand_debug(app);
    let app = app_with_subcommand_sql(app);
    let app = app_arg_value_added_tax_identification_number(app);
    let app = app_arg_legal_entity_identifier(app);
    let app = app_arg_ticker_symbol(app);
    let app = app_arg_international_standard_of_industrial_classification_v4(app);

    let matches = app.get_matches();

    let config = Config {
        verbose: pet_verbose(&matches),
        output_format: pet_output_format(&matches),
    };

    if      let Some(matches) = matches.subcommand_matches("count")  { run_subcommand_count(&config, matches) }
    else if let Some(matches) = matches.subcommand_matches("list")   { run_subcommand_list(&config, matches) }
    // else if let Some(matches) = matches.subcommand_matches("create") {} run_subcommand_create(&config, matches) }
    // else if let Some(matches) = matches.subcommand_matches("read")   { run_subcommand_read(&config, matches) }
    // else if let Some(matches) = matches.subcommand_matches("update") { run_subcommand_update(&config, matches) }
    // else if let Some(matches) = matches.subcommand_matches("delete") { run_subcommand_delete(&config, matches) }
    else if let Some(matches) = matches.subcommand_matches("db")     { run_subcommand_db(&config, matches) }
    else if let Some(matches) = matches.subcommand_matches("debug")  { run_subcommand_debug(&config, matches) }
    else if let Some(matches) = matches.subcommand_matches("sql")    { run_subcommand_sql(&config, matches) };
}
