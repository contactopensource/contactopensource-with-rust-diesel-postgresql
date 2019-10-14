#![recursion_limit="1024"]

extern crate clap;
extern crate contactopensource;
extern crate diesel;
extern crate diesel_dynamic_schema;
extern crate bigdecimal;
extern crate chrono;
//#[macro_use] extern crate lazy_static;
#[macro_use] extern crate maplit;
extern crate r2d2;
extern crate rand;
extern crate uuid;

use ::clap::{App, Arg, ArgMatches, SubCommand};
use ::diesel::prelude::*;
//use ::std::collections::HashMap;
use ::uuid::Uuid;

use ::contactopensource::{schema}; //TODO add models, traits
use ::contactopensource::traits::as_serde_json_value::AsSerdeJsonValue;
use ::contactopensource::traits::clap_able::ClapAble;
//use ::contactopensource::traits::fab_able::FabAble;

use ::contactopensource::schema::items::table as table;
use ::contactopensource::models::item as t;
use t::item::Item as T;

enum TableEnum {
    Items(contactopensource::schema::items::table),
    Arcs(contactopensource::schema::arcs::table),
}

enum ActionEnum {
    Create,
    Read,
    Update,
    Delete,
    Count,
    List,
}

enum OutputFormatEnum {
    Text,
    JSON,
}

struct Config {
    verbose: bool,
    output_format: OutputFormatEnum,
}

////
//
// App args that are for typical apps
//
////

fn arg_verbose<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("verbose")
        .help("Verbose output.")
        .short("v")
        .long("verbose")
}

fn arg_output_format<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("output_format")
        .help("Output format; examples: \"text\", \"json\".")
        .long("output_format")
        .value_name("FORMAT")
        .takes_value(true)
}

////
//
// App args that are data actions
//
////

fn arg_count<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("count")
        .help("Count, such as count a table's records")
        .long("count")
        .requires("table")
}

fn arg_list<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("list")
        .help("List, such as list a table's record")
        .requires("table")
}

fn arg_create<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("create")
        .help("Create, such as create a table record")
        .requires("table")
}

fn arg_read<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("read")
        .help("Read, such as read a table record")
        .requires("table")
        .requires("id")
}

pub fn arg_update<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("update")
        .help("Update a table record")
        .requires("table")
        .requires("id")
}

pub fn arg_delete<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("delete")
        .help("Delete a table record")
        .requires("table")
        .requires("id")
}

////
//
// App args that are data identifiers
//
////

fn arg_table<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("table")
        .help("Table name.\nExample: \"persons\"")
        .long("table")
        .value_name("NAME")
        .takes_value(true)
}

fn arg_id<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("id")
        .help("Id as a 32-character lowercase hexadecimal.\nExample: 4d8453c1d45b045c6716699326c7b7fb")
        .long("id")
        .value_name("ZID")
        .takes_value(true)
}

fn arg_tenant_id<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("tenant_id")
        .help("Tenant of the data, such as for multi-tenant systems.\nExample: 7bd380209cd310d3ad4e7f980298cbea")
        .long("tenant_id")
        .value_name("ZID")
        .takes_value(true)
    }

fn arg_typecast<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("typecast")
        .help("Typecast of the data, such as for single table inheritance.\nExample: \"friend\"")
        .long("typecast")
        .value_name("TEXT")
        .takes_value(true)
}

fn arg_state<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("state")
        .help("State of the data, such as for a state machine.\nExample: \"active\"")
        .long("state")
        .value_name("TEXT")
        .takes_value(true)
}

fn arg_updated_at_timestamp_utc<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("updated_at_timestamp_utc")
        .help("Updated at timestamp UTC.\nExample: \"2021-01-01T00:00:00Z\"")
        .long("updated_at_timestamp_utc")
        .value_name("TIMESTAMP")
        .takes_value(true)
}

fn arg_updated_at_clock_count<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("updated_at_clock_count")
        .help("Updated at clock counter.\nExample: \"123456789\"")
        .long("updated_at_clock_count")
        .value_name("COUNTER")
        .takes_value(true)
}

fn arg_updated_by_text<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("updated_by_text")
        .help("Updated by, in other words, typically freeform.\nExample: \"alice.anderson@example.com\"")
        .long("updated_by_text")
        .value_name("TEXT")
        .takes_value(true)
}

////
//
// App args that are content fields
//
////

fn arg_value_added_tax_identification_number<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("value_added_tax_identification_number")
        .help("Value added tax identification number.\nSee: https://schema.org/vatID\nSee: https://en.wikipedia.org/wiki/VAT_identification_number")
        .long("value_added_tax_identification_number")
        .value_name("TEXT")
        .takes_value(true)
}

fn arg_legal_entity_identifier<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("legal_entity_identifier")
        .help("Legal entity identifier.\nSee: https://schema.org/leiCode\nSee: https://en.wikipedia.org/wiki/Legal_Entity_Identifier")
        .long("legal_entity_identifier")
        .value_name("TEXT")
        .takes_value(true)
}

fn arg_ticker_symbol<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("ticker_symbol")
        .help("Ticker symbol.\nSee: https://en.wikipedia.org/wiki/Ticker_symbol")
        .long("ticker_symbol")
        .value_name("TEXT")
        .takes_value(true)
}

fn arg_international_standard_of_industrial_classification_v4<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("international_standard_of_industrial_classification_v4")
        .help("International Standard of Industrial Classification, Revision 4\nSee: https://schema.org/isicV4 https://en.wikipedia.org/wiki/International_Standard_Industrial_Classification")
        .long("international_standard_of_industrial_classification_v4")
        .value_name("TEXT")
        .takes_value(true)
}

////
//
// App subcommands that can help with debugging and diagnostics
//
////

pub fn app_dot_subcommand_db<'a, 'ar>(app: App<'a, 'ar>) -> App<'a, 'ar> {
    app.subcommand(
        SubCommand::with_name("db")
        .about("Database diagnotics")
        .help("Show database diagnostics, such as table names"))
}

pub fn app_dot_subcommand_debug<'a, 'ar>(app: App<'a, 'ar>) -> App<'a, 'ar> {
    app.subcommand(
        SubCommand::with_name("debug")
        .about("Debug")
        .help("Debug the application by showing diagnostics"))
}

pub fn app_dot_subcommand_sql<'a, 'ar>(app: App<'a, 'ar>) -> App<'a, 'ar> {
    app.subcommand(
        SubCommand::with_name("sql")
        .about("SQL command")
        .help("Run an arbitrary SQL command string -- use with caution"))
}

////
//
// Run action
//
////

//TODO make this work
// macro_rules! verbose {
//     ( $e:expr ) => {
//         if config.verbose { 
//             $e
//         }
//     }
// }

fn run_count(config: &Config, matches: &ArgMatches) {
    let action_name: &str = "count"; if config.verbose { output_action(action_name) };
    let table_name: &str = matches.value_of("table").unwrap(); if config.verbose { output_table(table_name) };

    if config.verbose { output_connect() };
    let connection = ::contactopensource::db_connection();

    if config.verbose { output_execute() };
    let result: QueryResult<i64> = table.count().get_result(&connection);
    let count: i64 = result
        .unwrap_or_else(|_|
            panic!("cannot {} {}", action_name, table_name)
        );
    println!("count:{}", count);
}

fn run_list(config: &Config, matches: &ArgMatches) {
    let action_name: &str = "list"; if config.verbose { output_action(action_name) };
    let table_name: &str = matches.value_of("table").unwrap(); if config.verbose { output_table(table_name) };

    if config.verbose { output_connect() };
    let connection = ::contactopensource::db_connection();

    if config.verbose { output_execute() };
    let xx: Vec<T> = table
        .load::<T>(&connection)
        .unwrap_or_else(|_|
            panic!("cannot {} {}", action_name, table_name)
        );
    for x in xx { output_x(config, x) };
}

fn run_create(config: &Config, matches: &ArgMatches) {
    let action_name: &str = "list"; if config.verbose { output_action(action_name) };
    let table_name: &str = matches.value_of("table").unwrap(); if config.verbose { output_table(table_name) };

    if config.verbose { output_connect() };
    let connection = ::contactopensource::db_connection();

    let x: T = T::from_clap_arg_matches(matches); if config.verbose { output_debug(&x) };

    if config.verbose { output_execute() };
    let x: T = diesel::insert_into(table)
        .values(&x)
        .get_result(&connection)
        .unwrap_or_else(|_|
            panic!("cannot {} {}", action_name, table_name)
        );
    output_x(&config, x);
}

fn run_read(config: &Config, matches: &ArgMatches) {
    let action_name: &str = "read"; if config.verbose { output_action(action_name) };
    let table_name: &str = matches.value_of("table").unwrap(); if config.verbose { output_table(table_name) };
    let id: Uuid = Uuid::parse_str(matches.value_of("id").unwrap()).unwrap(); if config.verbose { output_id(&id) };

    if config.verbose { output_connect() };
    let connection = ::contactopensource::db_connection();

    if config.verbose { output_execute() };
    let x: T = table.find(id).first::<T>(&connection)
        .unwrap_or_else(|_|
            panic!("cannot {} {}", action_name, table_name)
        );
    output_x(&config, x);
}

fn run_update(config: &Config, matches: &ArgMatches) {
    let action_name: &str = "update"; if config.verbose { output_action(action_name) };
    let table_name: &str = matches.value_of("table").unwrap(); if config.verbose { output_table(table_name) };
    let id: Uuid = Uuid::parse_str(matches.value_of("id").unwrap()).unwrap(); if config.verbose { output_id(&id) };

    if config.verbose { output_connect() };
    let connection = ::contactopensource::db_connection();

    let x: T = T::from_clap_arg_matches(matches); if config.verbose { output_debug(&x) };

    if config.verbose { output_execute() };
    let x: T = diesel::update(table.find(id))
       .set(&x)
       .get_result::<T>(&connection)
       .unwrap_or_else(|_|
            panic!("cannot {} {} {}", action_name, table_name, id)
        );
    output_x(&config, x);
}

fn run_delete(config: &Config, matches: &ArgMatches) {
    let action_name: &str = "delete"; if config.verbose { output_action(action_name) };
    let table_name: &str = matches.value_of("table").unwrap(); if config.verbose { output_table(table_name) };
    let id: Uuid = Uuid::parse_str(matches.value_of("id").unwrap()).unwrap(); if config.verbose { output_id(&id) };

    if config.verbose { output_connect() };
    let connection = ::contactopensource::db_connection();

    if config.verbose { output_execute() };
    let x: T = diesel::delete(table.filter(schema::items::id.eq(id)))
        .get_result::<T>(&connection)
        .unwrap_or_else(|_|
            panic!("cannot {} {} {}", action_name, table_name, id)
        );
    if config.verbose { output_x(&config, x) };
}

fn run_subcommand_db(config: &Config, _matches: &ArgMatches) {
    if config.verbose { output_subcommand("db") };
    println!("db"); // TODO replace with anything more useful

    // let table_name = diesel_dynamic_schema::table("contacts");
    // println!("table:{}", table_name);

    // let my_column= table_name.column::<pg::types::sql_types::Uuid, _>("id");
    // let my_column= table_name.column<pg::types::sql_types::Uuid, _>("id");
    // println!("column:{}", my_column);

    if config.verbose { output_connect() };
    let _connection = ::contactopensource::db_connection();
    //TODO
    // let tablenames: Vec<String> = diesel::sql_query("SELECT tablename FROM pg_catalog.pg_tables where schemaname = 'public' and tablename not like '\_\_%' ;").load::<(String)>(&conn);
    // println!("{:?}", table_name.select((my_column)).first(&connection))
}

fn run_subcommand_debug(config: &Config, _matches: &ArgMatches) {
    if config.verbose { output_subcommand("debug") };
    println!("debug"); // TODO replace with anything more useful

    println!("primary_key:{:?}", table.primary_key());

    let all_columns: <table as Table>::AllColumns = table::all_columns();
    println!("all_columns 0..5 {:?} {:?} {:?} {:?} {:?} {:?}",
    all_columns.0,
    all_columns.1,
    all_columns.2,
    all_columns.3,
    all_columns.4,
    all_columns.5,
    );

    // let ac: schema::items::SqlType = table::all_columns();
    // println!("ac {:?} {:?} {:?}",
    // ac.0,
    // ac.1,
    // ac.2,
    // );

    println!("star:{:?}", table.star());
    println!("star:{:?}", schema::items::columns::star);
}

fn run_subcommand_sql(config: &Config, _matches: &ArgMatches) {
    if config.verbose { output_subcommand("sql") };
    println!("sql"); // TODO replace with anything more useful

    if config.verbose { output_connect() };
    let _connection = ::contactopensource::db_connection();
    //TODO
    // results = diesel::sql_query("SELECT * FROM contacts ORDER BY id")
    // .load(&connection);

    // println!("{:?}", results)
}

////
//
// Helpers: parse input settings a.k.a. "pet" instead of "let", "get", "set".
//
////

// lazy_static! {
//     static ref ACTION_NAME_TO_ENUM: HashMap<String, ActionEnum> = {
//         hashmap![
//             String::from("count")  => ActionEnum::Count,
//             String::from("list")   => ActionEnum::List,
//             String::from("create") => ActionEnum::Create,
//             String::from("read")   => ActionEnum::Read,
//             String::from("update") => ActionEnum::Update,
//             String::from("delete") => ActionEnum::Delete,
//         ]
//     };
// }

// lazy_static! {
//     static ref TABLE_NAME_TO_ENUM: HashMap<String, TableEnum> = {
//         hashmap![
//             String::from("items") => TableEnum::Items,
//             String::from("arcs")  => TableEnum::Arcs,
//         ]
//     };
// }

// lazy_static! {
//     static ref OUTPUT_FORMAT_NAME_TO_ENUM: HashMap<String, OutputFormatEnum> = {
//         hashmap![
//             String::from("text") => OutputFormatEnum::Text,
//             String::from("json") => OutputFormatEnum::JSON,
//         ]
//     };
// }

fn pet_verbose(matches: &ArgMatches) -> bool {
    matches.is_present("verbose")
}

fn pet_output_format(matches: &ArgMatches) -> OutputFormatEnum {
    match matches.value_of("output_format") {
        Some("text") => OutputFormatEnum::Text,
        Some("json") => OutputFormatEnum::JSON,
        Some(_) => OutputFormatEnum::Text,
        None => OutputFormatEnum::Text,
    }
}

//TODO experiment
// fn pet_table(matches: &ArgMatches) -> Box<dyn FabAble> {
//     if let name = matches.value_of("table").unwrap() {
//         match name {
//             "items" => Box::new(contactopensource::models::item::item::Item) as Box<dyn FabAble>,
//             "arcs" => Box::new(contactopensource::models::arc::arc::Arc) as Box<dyn FabAble>,
//             _ => Box::new(contactopensource::models::item::item::Item) as Box<dyn FabAble>,
//         }
//     };
//     Box::new(contactopensource::models::item::item::Item) as Box<dyn FabAble>
// }

////
//
// Helpers: output
//
////

#[allow(dead_code)]
fn output(s: &str){
    println!("{}", s);
}

#[allow(dead_code)]
fn output_debug(x: &dyn std::fmt::Debug){
    println!("{:#?}", x);
}

#[allow(dead_code)]
fn output_subcommand(s: &str){
    println!("subcommand:{}", s);
}

#[allow(dead_code)]
fn output_connect(){
    println!("connect...");
}

#[allow(dead_code)]
fn output_execute(){
    println!("execute...");
}

#[allow(dead_code)]
fn output_action(action_name: &str){
    println!("action:{}", action_name);
}

#[allow(dead_code)]
fn output_table(table_name: &str){
    println!("table:{}", table_name);
}

#[allow(dead_code)]
fn output_id(id: &Uuid) {
    println!("id:{}", id.to_simple())
}

#[allow(dead_code)]
fn output_count(my_count: usize) {
    println!("count:{}", my_count)
}

#[allow(dead_code)]
fn output_x<T: std::fmt::Debug + AsSerdeJsonValue>(config: &Config, x: T) {
    match config.output_format {
        OutputFormatEnum::Text => {
            println!("{:?}", x)
        },
        OutputFormatEnum::JSON => {
            println!("{:?}", x.as_serde_json_value().to_string())
        },
    }
}

////
//
// Dispatch all table action commands.
//
// TODO: refactor to dynamic dispatch
////

fn dispatch(config: &Config, matches: &ArgMatches) {
    if !matches.is_present("table") { return };
    if !matches.is_present("action") { return };
    
    let table_name: &str = matches.value_of("table").unwrap(); if config.verbose { output_table(table_name) };
    //let table_enum: TableEnum = TABLE_NAME_TO_ENUM[&String::from(table_name)];
    let table_enum: TableEnum = match table_name {
        "items" => TableEnum::Items(contactopensource::schema::items::table),
        "arcs" => TableEnum::Arcs(contactopensource::schema::arcs::table),
        _ => return,
    };

    let action_name: &str = matches.value_of("action").unwrap(); if config.verbose { output_action(action_name) };
    //let action_enum: ActionEnum = ACTION_NAME_TO_ENUM[&String::from(action_name)];
    let action_enum: ActionEnum = match action_name {
        "count" => ActionEnum::Count,
        "list" => ActionEnum::List,
        _ => return,
    };

    // match table_enum {
    //     TableEnum::Items => {
    //         match action_enum { 
    //             ActionEnum::Count => {
    //                 let connection = contactopensource::db_connection();
    //                 let result: QueryResult<i64> = contactopensource::schema::items::table.count().get_result(&connection);
    //                 let count: i64 = result.unwrap_or_else(|_| panic!("cannot {} {}", action_name, table_name));
    //                 println!("count:{}", count);
    //             },
    //             ActionEnum::List => {
    //                 let connection = contactopensource::db_connection();
    //                 let result: Result<Vec<contactopensource::models::item::item::Item>, diesel::result::Error> = contactopensource::schema::items::table.load::<contactopensource::models::item::item::Item>(&connection);
    //                 let xx: Vec<contactopensource::models::item::item::Item> = result.unwrap_or_else(|_| panic!("cannot {} {}", action_name, table_name));
    //                 for x in xx { output_x(config, x) };
    //             },
    //             _ => {
    //                 panic!("Cannot dispatch because action is unrecognized; action_name:{}", action_name);
    //             },
    //         };
    //     },
    //     _ => {
    //         panic!("Cannot dispatch because table is unrecognized; table_name:{}", table_name);
    //     },
    // };

    // match table_enum {
    //     TableEnum::Items(table) => {
    //         match action_enum { 
    //             ActionEnum::Count => {
    //                 let connection = contactopensource::db_connection();
    //                 let result: QueryResult<i64> = table.count().get_result(&connection);
    //                 let count: i64 = result.unwrap_or_else(|_| panic!("cannot {} {}", action_name, table_name));
    //                 println!("count:{}", count);
    //             },
    //             ActionEnum::List => {
    //                 let connection = contactopensource::db_connection();
    //                 let result: Result<Vec<contactopensource::models::item::item::Item>, diesel::result::Error> = table.load::<contactopensource::models::item::item::Item>(&connection);
    //                 let xx: Vec<contactopensource::models::item::item::Item> = result.unwrap_or_else(|_| panic!("cannot {} {}", action_name, table_name));
    //                 for x in xx { output_x(config, x) };
    //             },
    //             _ => {
    //                 panic!("Cannot dispatch because action is unrecognized; action_name:{}", action_name);
    //             },
    //         };
    //     },
    //     _ => {
    //         panic!("Cannot dispatch because table is unrecognized; table_name:{}", table_name);
    //     },
    // };

}

fn main() {
    let app = App::new("ContactOpenSource")
        .version("1.0.0")
        .author("Joel Parker Henderson <joel@contactopensource.com>")
        .about("ContactOpenSource is contact relationship manager address book, open source, and free.");
    
    // Typical args
    let app = app.arg(arg_verbose());
    let app = app.arg(arg_output_format());

    // Data actions
    let app = app.arg(arg_count());
    let app = app.arg(arg_list());
    let app = app.arg(arg_create());
    let app = app.arg(arg_read());
    let app = app.arg(arg_update());
    let app = app.arg(arg_delete());

    // Data identifiers
    let app = app.arg(arg_table());
    let app = app.arg(arg_id());
    let app = app.arg(arg_tenant_id());
    let app = app.arg(arg_typecast());
    let app = app.arg(arg_state());
    let app = app.arg(arg_updated_at_timestamp_utc());
    let app = app.arg(arg_updated_at_clock_count());
    let app = app.arg(arg_updated_by_text());

    // Content fields
    let app = app.arg(arg_value_added_tax_identification_number());
    let app = app.arg(arg_legal_entity_identifier());
    let app = app.arg(arg_ticker_symbol());
    let app = app.arg(arg_international_standard_of_industrial_classification_v4());
    
    // Subcommands
    let app = app_dot_subcommand_db(app);
    let app = app_dot_subcommand_debug(app);
    let app = app_dot_subcommand_sql(app);

    let matches = app.get_matches();

    let config = Config {
        verbose: pet_verbose(&matches),
        output_format: pet_output_format(&matches),
    };

    if      matches.is_present("count")  { run_count(&config, &matches) }
    else if matches.is_present("list")   { run_list(&config, &matches) }
    else if matches.is_present("create") { run_create(&config, &matches) }
    else if matches.is_present("read")   { run_read(&config, &matches) }
    else if matches.is_present("update") { run_update(&config, &matches) }
    else if matches.is_present("delete") { run_delete(&config, &matches) }
    else if let Some(matches) = matches.subcommand_matches("db")     { run_subcommand_db(&config, matches) }
    else if let Some(matches) = matches.subcommand_matches("debug")  { run_subcommand_debug(&config, matches) }
    else if let Some(matches) = matches.subcommand_matches("sql")    { run_subcommand_sql(&config, matches) };
}
