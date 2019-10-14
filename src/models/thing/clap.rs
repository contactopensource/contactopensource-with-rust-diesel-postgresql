use clap::{App, Arg, ArgMatches};
use crate::types;
use crate::models::thing::thing::Thing as T;

impl T {


    fn arg_name<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("name")
            .help("name; example: \"book\"")
            .long("name")
            .value_name("NAME")
            .takes_value(true)
    }

    fn arg_start_timestamp_utc<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("start_timestamp_utc")
            .help("start_timestamp_utc; example: \"2021-01-01T00:00:00Z\"")
            .long("start_timestamp_utc")
            .value_name("TIMESTAMP")
            .takes_value(true)
    }

    fn arg_stop_timestamp_utc<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("stop_timestamp_utc")
            .help("stop_timestamp_utc; example: \"2021-01-01T00:00:00Z\"")
            .long("stop_timestamp_utc")
            .value_name("TIMESTAMP")
            .takes_value(true)
    }

    fn arg_duration_as_seconds<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("duration_as_seconds")
            .help("duration_as_seconds; example: 3600 seconds is 1 hour")
            .long("duration_as_seconds")
            .value_name("SECONDS")
            .takes_value(true)
    }

}

impl crate::traits::clap_able::ClapAble for T {

    fn init_clap_app<'a, 'ar>(app: App<'a, 'ar>) -> App<'a, 'ar> {
        let app = app.arg(Self::arg_name());
        let app = app.arg(Self::arg_start_timestamp_utc());
        let app = app.arg(Self::arg_stop_timestamp_utc());
        let app = app.arg(Self::arg_duration_as_seconds());
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

            // Name-related
            name: types::text::from_option_str(matches.value_of("name")),
        }
    }

}
