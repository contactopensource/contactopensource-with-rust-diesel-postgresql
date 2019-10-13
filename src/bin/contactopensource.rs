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
use ::uuid::Uuid;
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
// App args that are for typical apps
//
////

fn arg_verbose<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("verbose")
        .help("Verbose output.")
        .short("v")
        .long("verbose")
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
// App args that are output formats
//
////

fn arg_output_text<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("output-text")
        .help("Output format is text.")
        .long("output-text")
}

fn arg_output_json<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("output-json")
        .help("Output format is JSON.")
        .long("output-json")
}

fn arg_output_html<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("output-html")
        .help("Output format is HTML.")
        .long("output-html")
}

fn arg_output_xml<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("output-xml")
        .help("Output format is XML.")
        .long("output-xml")
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
    let my_action: &str = "count"; if config.verbose { output_action(my_action) };
    let my_table: &str = matches.value_of("table").unwrap(); if config.verbose { output_table(my_table) };

    if config.verbose { output_connect() };
    let connection = establish_connection();

    if config.verbose { output_execute() };
    let xx = schema::contacts::table
        .load::<Contact>(&connection)
        .unwrap_or_else(|_|
            panic!("cannot {} {}", my_action, my_table)
        );
    println!("count:{}", xx.len());
}

fn run_list(config: &Config, matches: &ArgMatches) {
    let my_action: &str = "list"; if config.verbose { output_action(my_action) };
    let my_table: &str = matches.value_of("table").unwrap(); if config.verbose { output_table(my_table) };

    if config.verbose { output_connect() };
    let connection = establish_connection();

    if config.verbose { output_execute() };
    let xx: Vec<Contact> = schema::contacts::table
        .load::<Contact>(&connection)
        .unwrap_or_else(|_|
            panic!("cannot {} {}", my_action, my_table)
        );
    for x in xx { output_x(config, x) }
}

fn run_create(config: &Config, matches: &ArgMatches) {
    let my_action: &str = "list"; if config.verbose { output_action(my_action) };
    let my_table: &str = matches.value_of("table").unwrap(); if config.verbose { output_table(my_table) };

    if config.verbose { output_connect() };
    let connection = establish_connection();

    output("TODO");
    // if config.verbose { output_execute() };
    // let x = diesel::insert_into(schema::contacts::table)
    //     .values(&insertable) //TODO
    //     .get_result(&connection)
    //     .unwrap_or_else(|_|
    //         panic!("cannot {} {}", my_action, my_table)
    //     );
    // output_x(&config, x);
}

fn run_read(config: &Config, matches: &ArgMatches) {
    let my_action: &str = "read"; if config.verbose { output_action(my_action) };
    let my_table: &str = matches.value_of("table").unwrap(); if config.verbose { output_table(my_table) };
    let my_id: Uuid = Uuid::parse_str(matches.value_of("id").unwrap()).unwrap(); if config.verbose { output_id(&my_id) };

    if config.verbose { output_connect() };
    let connection = establish_connection();

    output("TODO");
    // if config.verbose { output_execute() };
    // let x = schema::contacts::table.find(my_id).first::<Contact>(&connection)
    //     .unwrap_or_else(|_|
    //         panic!("cannot {} {}", my_action, my_table)
    //     );
    // output_x(&config, x);
}

fn run_update(config: &Config, matches: &ArgMatches) {
    let my_action: &str = "update"; if config.verbose { output_action(my_action) };
    let my_table: &str = matches.value_of("table").unwrap(); if config.verbose { output_table(my_table) };
    let my_id: Uuid = Uuid::parse_str(matches.value_of("id").unwrap()).unwrap(); if config.verbose { output_id(&my_id) };

    if config.verbose { output_connect() };
    let connection = establish_connection();

    output("TODO");
    // if config.verbose { output_execute() };
    // let x = schema::contacts::table.find(my_id).first::<Contact>(&connection)
    //    .set(_parsed)
    //    .get_result::<Contact>(&connection)
    //    .unwrap_or_else(|_|
    //         panic!("cannot {} {} {}", my_action, my_table, my_arg)
    //     );
    // output_x(&config, x);
}

fn run_delete(config: &Config, matches: &ArgMatches) {
    let my_action: &str = "delete"; if config.verbose { output_action(my_action) };
    let my_table: &str = matches.value_of("table").unwrap(); if config.verbose { output_table(my_table) };
    let my_id: Uuid = Uuid::parse_str(matches.value_of("id").unwrap()).unwrap(); if config.verbose { output_id(&my_id) };

    if config.verbose { output_connect() };
    let connection = establish_connection();

    if config.verbose { output_execute() };
    let x = diesel::delete(schema::contacts::table.filter(schema::contacts::id.eq(my_id)))
        .get_result::<Contact>(&connection)
        .unwrap_or_else(|_|
            panic!("cannot {} {} {}", my_action, my_table, my_id)
        );
    if config.verbose { output_x(&config, x) };
}

fn run_subcommand_db(config: &Config, _matches: &ArgMatches) {
    if config.verbose { output_subcommand("db") };
    println!("db"); // TODO replace with anything more useful

    // let my_table = diesel_dynamic_schema::table("contacts");
    // println!("table:{}", my_table);

    // let my_column= my_table.column::<pg::types::sql_types::Uuid, _>("id");
    // let my_column= my_table.column<pg::types::sql_types::Uuid, _>("id");
    // println!("column:{}", my_column);

    if config.verbose { output_connect() };
    let _connection = establish_connection();
    //TODO
    // let tablenames: Vec<String> = diesel::sql_query("SELECT tablename FROM pg_catalog.pg_tables where schemaname = 'public' and tablename not like '\_\_%' ;").load::<(String)>(&conn);
    // println!("{:?}", my_table.select((my_column)).first(&connection))
}

fn run_subcommand_debug(config: &Config, _matches: &ArgMatches) {
    if config.verbose { output_subcommand("debug") };
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
    if config.verbose { output_subcommand("sql") };
    println!("sql"); // TODO replace with anything more useful

    if config.verbose { output_connect() };
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

fn output(s: &str){
    println!("{}", s);
}

fn output_subcommand(s: &str){
    println!("subcommand:{}", s);
}

fn output_connect(){
    println!("connect...");
}

fn output_execute(){
    println!("execute...");
}

fn output_action(my_action: &str){
    println!("action:{}", my_action);
}

fn output_table(my_table: &str){
    println!("table:{}", my_table);
}

fn output_id(my_id: &Uuid) {
    println!("id:{}", my_id.to_simple())
}

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
    
    // Typical args
    let app = app.arg(arg_verbose());

    // Output formats
    let app = app.arg(arg_output_text());
    let app = app.arg(arg_output_json());
    let app = app.arg(arg_output_html());
    let app = app.arg(arg_output_xml());

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
