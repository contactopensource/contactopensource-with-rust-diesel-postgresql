use clap::{App, Arg, ArgMatches};
use crate::types;
use crate::models::item::item::Item as T;

impl crate::traits::clap_able::ClapAble for T {

    fn init_clap_app<'a, 'ar>(app: App<'a, 'ar>) -> App<'a, 'ar> {
        app
        .arg(Arg::with_name("uri")
            .help("URI; example: \"https://example.com/example.txt\"")
            .long("uri")
            .value_name("URI")
            .takes_value(true))
        .arg(Arg::with_name("text")
            .help("text; example: \"hello world\"")
            .long("text")
            .value_name("TEXT")
            .takes_value(true))
        .arg(Arg::with_name("json")
            .help("JSON; example: \"{\"hello\":\"world\"}")
            .long("json")
            .value_name("JSON")
            .takes_value(true))
        .arg(Arg::with_name("xml")
            .help("XML; example: \"<?xml version=\"1.0\"?><example>hello world</example>")
            .long("xml")
            .value_name("XML")
            .takes_value(true))
        .arg(Arg::with_name("number")
            .help("number; example: 1234.5678")
            .long("number")
            .value_name("NUMBER")
            .takes_value(true))
    }

    fn from_clap_arg_matches(matches: &ArgMatches) -> T {
        T {
            id: types::id::from_option_str(matches.value_of("id")).unwrap(),

            // Programming-related
            tenant_id: types::id::from_option_str(matches.value_of("tenant_id")),
            typecast: types::typecast::from_option_str(matches.value_of("typecast")),
            state: types::state::from_option_str(matches.value_of("state")),

            // Update-related
            updated_at_timestamp_utc: types::timestamp::from_option_str(matches.value_of("updated_at_timestamp_utc")),
            updated_at_clock_count: types::count::from_option_str(matches.value_of("updated_at_clock_count")),
            updated_by_text: types::text::from_option_str(matches.value_of("updated_by_text")),

            // Meta-related
            uri: types::uri_string::from_option_str(matches.value_of("uri")),

            // Content-related
            text: types::text::from_option_str(matches.value_of("text")),
            json: types::json_value::from_option_str(matches.value_of("json")),
            xml: types::xml_string::from_option_str(matches.value_of("xml")),
            number: types::number::from_option_str(matches.value_of("number")),
        }
    }

}
