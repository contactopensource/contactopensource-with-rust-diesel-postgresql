use clap::{App, Arg, ArgMatches};
use crate::types;
use crate::models::link_contact::link_contact::LinkContact as T;

impl crate::traits::clap_able::ClapAble for T {

    fn init_clap_app<'a, 'ar>(app: App<'a, 'ar>) -> App<'a, 'ar> {
        let app = app.arg(Self::arg_label());
        let app = app.arg(Self::arg_uri());
        app
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

            // Link-related
            label: types::label::from_option_str(matches.value_of("label")),
            uri: types::uri_string::from_option_str(matches.value_of("uri")),
        }
    }

}

impl T {

    fn arg_label<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("label")
            .help("label; example: \"Example web page\"")
            .long("address")
            .value_name("TEXT")
            .takes_value(true)
    }

    fn arg_uri<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("uri")
            .help("URI; example: \"https://example.com/example.html\"")
            .long("uri")
            .value_name("TEXT")
            .takes_value(true)
    }

}
