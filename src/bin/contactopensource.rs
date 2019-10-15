#![recursion_limit="1024"]

extern crate clap;
extern crate contactopensource;
extern crate diesel;
extern crate diesel_dynamic_schema;
extern crate bigdecimal;
extern crate chrono;
//#[macro_use] extern crate lazy_static;
//#[macro_use] extern crate maplit;
extern crate r2d2;
extern crate rand;
extern crate uuid;

use ::clap::{App, Arg, ArgMatches, SubCommand};
use ::diesel::prelude::*;
//use ::phf;
//use ::std::collections::HashMap;
use ::uuid::Uuid;
use ::contactopensource::traits::{as_serde_json_value::*, clap_able::*, db_crud::*};
use ::contactopensource::enums::{action_enum::*, /*model_enum::*,*/ output_format_enum::*, table_enum::*};
use ::contactopensource::{schema, models, /*types*/};

struct Config {
    verbose: bool,
    output_format: Option<OutputFormatEnumType>,
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
// App args that are data tables and models
//
////

fn arg_item<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("item")
        .help("Item table.")
        .long("item")
}

fn arg_arc<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("arc")
        .help("Arc table.")
        .long("arc")
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
}

fn arg_list<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("list")
        .help("List, such as list a table's record")
        .long("list")
}

fn arg_create<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("create")
        .help("Create, such as create a table record")
        .long("create")
}

fn arg_read<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("read")
        .help("Read, such as read a table record")
        .long("read")
}

pub fn arg_update<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("update")
        .help("Update a table record")
        .long("update")
}

pub fn arg_delete<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("delete")
        .help("Delete a table record")
        .long("delete")
}

////
//
// App args that are data fields
//
////

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

    use schema::items::table as my_table; //TODO generalize
    use schema::items::columns as my_columns; //TODO generalize

    println!("primary_key:{:?}", my_table.primary_key());

    let all_columns: <my_table as Table>::AllColumns = my_table::all_columns();
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

    println!("star:{:?}", my_table.star());
    println!("star:{:?}", my_columns::star);
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

//TODO archive
// fn pet_table_name_to_enum(s: &str) -> Option<TableEnum> {
//     match s {
//         "items" => Some(TABLE_ENUM_ITEMS(my_table)),
//         "arcs" => Some(TABLE_ENUM_ARCS(schema::arcs::table)),
//         _ => None,
//     }
// }

fn pet_table_matches_to_enum(matches: &ArgMatches) -> Option<TableEnumType> {
    if matches.is_present("item") { Some(TABLE_ENUM_ITEMS) }
    else if matches.is_present("arc") { Some(TABLE_ENUM_ARCS) }
    else { None }
}

//TODO archive
// fn pet_action_name_to_enum(s: &str) -> Option<ActionEnum> {
//     match s {
//         "count" => Some(ACTION_ENUM_COUNT),
//         "list" => Some(ACTION_ENUM_LIST),
//         _ => None,
//     }
// }

fn pet_action_matches_to_enum(matches: &ArgMatches) -> Option<ActionEnumType> {
    if matches.is_present("count") { Some(ACTION_ENUM_COUNT) }
    else if matches.is_present("list") { Some(ACTION_ENUM_LIST) }
    else { None }
}

fn pet_output_format_name_to_enum(s: &str) -> Option<OutputFormatEnumType> {
    match s {
        "text" => Some(OUTPUT_FORMAT_ENUM_TEXT),
        "json" => Some(OUTPUT_FORMAT_ENUM_JSON),
        _ => None,
    }
}

fn pet_output_format_matches_to_enum(matches: &ArgMatches) -> Option<OutputFormatEnumType> {
    match matches.value_of("output_format") {
        Some(s) => pet_output_format_name_to_enum(s),
        _ => None,
    }
}

fn pet_verbose(matches: &ArgMatches) -> bool {
    matches.is_present("verbose")
}

//TODO experiment
// fn pet_table(matches: &ArgMatches) -> Box<dyn FabAble> {
//     if let name = matches.value_of("table").unwrap() {
//         match name {
//             "items" => Box::new(models::item::item::Item) as Box<dyn FabAble>,
//             "arcs" => Box::new(models::arc::arc::Arc) as Box<dyn FabAble>,
//             _ => Box::new(models::item::item::Item) as Box<dyn FabAble>,
//         }
//     };
//     Box::new(models::item::item::Item) as Box<dyn FabAble>
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
fn output_count_i64(my_count: i64) {
    println!("count:{}", my_count)
}

#[allow(dead_code)]
fn output_x<T: std::fmt::Debug + AsSerdeJsonValue>(config: &Config, x: T) {
    match config.output_format {
        Some(OUTPUT_FORMAT_ENUM_TEXT) => {
            println!("{:?}", x)
        },
        Some(OUTPUT_FORMAT_ENUM_JSON) => {
            println!("{:?}", x.as_serde_json_value().to_string())
        },
        _ => {
            println!("{:?}", x)
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
    if config.verbose { output("dispatch") };

    let table_enum: TableEnumType = match pet_table_matches_to_enum(matches) {
        Some(x) => x,
        _ => { return },
    };
    if config.verbose { output("dispatch has table") };

    let action_enum: ActionEnumType = match pet_action_matches_to_enum(matches) {
        Some(x) => x,
        _ => { return }
    };
    if config.verbose { output("dispatch has action") };
    
    let action_name: &str = match action_enum {
        ACTION_ENUM_COUNT =>  "count",
        ACTION_ENUM_LIST   =>  "list",
        ACTION_ENUM_CREATE => "create",
        ACTION_ENUM_READ   => "read",
        ACTION_ENUM_UPDATE => "update",
        ACTION_ENUM_DELETE => "delete",
        _ => { panic!("?") }
     };
    if config.verbose { output_action(action_name) };

    match table_enum {
        TABLE_ENUM_ITEMS => {
            use schema::items::table as my_table;
            //use schema::items::columns as my_columns;
            use models::item::item::Item as my_model;
            let table_name = "items"; if config.verbose { output_table(table_name) };
            match action_enum { 
                ACTION_ENUM_COUNT => {
                    let result: QueryResult<i64> = my_model::db_count();
                    let count: i64 = result.unwrap_or_else(|_| panic!("cannot {} {}", action_name, table_name));
                    output_count_i64(count);
                },
                ACTION_ENUM_LIST => {
                    let connection = contactopensource::db_connection();
                    let result: Result<Vec<my_model>, diesel::result::Error> = my_table.load::<my_model>(&connection);
                    let xx: Vec<my_model> = result.unwrap_or_else(|_| panic!("cannot {} {}", action_name, table_name));
                    for x in xx { output_x(&config, x) };
                },
                ACTION_ENUM_CREATE => {
                    let insertable: my_model = my_model::from_clap_arg_matches(matches); if config.verbose { output_debug(&insertable) };
                    let connection = contactopensource::db_connection();
                    let result: QueryResult<usize> = diesel::insert_into(my_table).values(&insertable).execute(&connection);
                    let count: usize = result.unwrap_or_else(|_| panic!("cannot {} {}", action_name, table_name));
                    output_count(count);
                },
                ACTION_ENUM_READ => {
                    let id: Uuid = Uuid::parse_str(matches.value_of("id").unwrap()).unwrap(); if config.verbose { output_id(&id) };
                    let connection = contactopensource::db_connection();
                    let result: Result<my_model, diesel::result::Error> = my_table.find(id).first::<my_model>(&connection);
                    let x: my_model= result.unwrap_or_else(|_| panic!("cannot {} {}", action_name, table_name));
                    output_x(&config, x);
                },
                ACTION_ENUM_UPDATE => {
                    //TODO
                    // let id: Uuid = Uuid::parse_str(matches.value_of("id").unwrap()).unwrap(); if config.verbose { output_id(&id) };
                    // let changesettable: my_model = my_model::from_clap_arg_matches(matches); if config.verbose { output_debug(&changesettable) };
                    // let connection = contactopensource::db_connection();
                    // let result: QueryResult<usize> = diesel::update(my_table.find(id)).set(&changesetttable).execute(&connection);
                    // let count: usize = result.unwrap_or_else(|_| panic!("cannot {} {}", action_name, table_name));
                    // if config.verbose { output_count(count) };
                },
                ACTION_ENUM_DELETE => {
                    //TODO
                    // let id: Uuid = Uuid::parse_str(matches.value_of("id").unwrap()).unwrap(); if config.verbose { output_id(&id) };
                    // let connection = contactopensource::db_connection();
                    //let result: QueryResult<usize> = diesel::delete(my_table.find(my_columns::id.eq(id))).execute(&connection);
                    // let count: usize = result.unwrap_or_else(|_| panic!("cannot {} {}", action_name, table_name));
                    // if config.verbose { output_count(count) };
                },
                _ => (),
            };
        },
        TABLE_ENUM_ARCS => {
            use schema::arcs::table as my_table;
            //use schema::arcs::columns as my_columns;
            use models::arc::arc::Arc as my_model;
            let table_name = "arcs"; if config.verbose { output_table(table_name) };
            match action_enum { 
                ACTION_ENUM_COUNT => {
                    let result: QueryResult<i64> = my_model::db_count();
                    let count: i64 = result.unwrap_or_else(|_| panic!("cannot {} {}", action_name, table_name));
                    output_count_i64(count);
                },
                ACTION_ENUM_LIST => {
                    let connection = contactopensource::db_connection();
                    let result: Result<Vec<my_model>, diesel::result::Error> = my_table.load::<my_model>(&connection);
                    let xx: Vec<my_model> = result.unwrap_or_else(|_| panic!("cannot {} {}", action_name, table_name));
                    for x in xx { output_x(&config, x) };
                },
                ACTION_ENUM_CREATE => {
                    let insertable: my_model = my_model::from_clap_arg_matches(matches); if config.verbose { output_debug(&insertable) };
                    let connection = contactopensource::db_connection();
                    let result: QueryResult<usize> = diesel::insert_into(my_table).values(&insertable).execute(&connection);
                    let count: usize = result.unwrap_or_else(|_| panic!("cannot {} {}", action_name, table_name));
                    if config.verbose { output_count(count); }
                },
                ACTION_ENUM_READ => {
                    let id: Uuid = Uuid::parse_str(matches.value_of("id").unwrap()).unwrap(); if config.verbose { output_id(&id) };
                    let connection = contactopensource::db_connection();
                    let result: Result<my_model, diesel::result::Error> = my_table.find(id).first::<my_model>(&connection);
                    let x: my_model= result.unwrap_or_else(|_| panic!("cannot {} {}", action_name, table_name));
                    output_x(&config, x);
                },
                ACTION_ENUM_UPDATE => {
                    //TODO
                    // let connection = contactopensource::db_connection();
                    //let id: Uuid = Uuid::parse_str(matches.value_of("id").unwrap()).unwrap(); if config.verbose { output_id(&id) };
                },
                ACTION_ENUM_DELETE => {
                    //TODO
                    // let id: Uuid = Uuid::parse_str(matches.value_of("id").unwrap()).unwrap(); if config.verbose { output_id(&id) };
                    //let connection = contactopensource::db_connection();
                },
                _ => (),
            };
        },
        _ => (),
    };

}

fn main() {
    let app = App::new("ContactOpenSource")
        .version("1.0.0")
        .author("Joel Parker Henderson <joel@contactopensource.com>")
        .about("ContactOpenSource is contact relationship manager address book, open source, and free.");
    
    // Typical args
    let app = app.arg(arg_verbose());
    let app = app.arg(arg_output_format());

    // Data models
    let app = app.arg(arg_item());
    let app = app.arg(arg_arc());

    // Data actions
    let app = app.arg(arg_count());
    let app = app.arg(arg_list());
    let app = app.arg(arg_create());
    let app = app.arg(arg_read());
    let app = app.arg(arg_update());
    let app = app.arg(arg_delete());

    // Data ubiquitous fields
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
    println!("main:");
    println!("{:#?}", matches);

    let config = Config {
        verbose: pet_verbose(&matches),
        output_format: pet_output_format_matches_to_enum(&matches),
    };

    println!("before dispatch");
    println!("{:#?}", matches);
    dispatch(&config, &matches);

    if      let Some(matches) = matches.subcommand_matches("db")     { run_subcommand_db(&config, matches) }
    else if let Some(matches) = matches.subcommand_matches("debug")  { run_subcommand_debug(&config, matches) }
    else if let Some(matches) = matches.subcommand_matches("sql")    { run_subcommand_sql(&config, matches) };
}
