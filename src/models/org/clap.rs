use clap::{App, Arg, ArgMatches};
use crate::types;
use crate::models::org::org::Org as T;

impl T {

    fn arg_start_date<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("start_at_timestamp_utc")
            .help("start at timestamp; example: \"2021-01-01\"")
            .long("start_date")
            .value_name("DATE")
            .takes_value(true)
    }

    fn arg_stop_date<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("stop_date")
            .help("stop at date; example: \"2021-01-01\"")
            .long("stop_date")
            .value_name("DATE")
            .takes_value(true)
    }

}

impl crate::traits::clap_able::ClapAble for T {

    fn init_clap_app<'a, 'ar>(app: App<'a, 'ar>) -> App<'a, 'ar> {
        let app = app.arg(Self::arg_start_date());
        let app = app.arg(Self::arg_stop_date());
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

            // Lifetime-related
            start_date: types::date::from_option_str(matches.value_of("start_date")),
            stop_date: types::date::from_option_str(matches.value_of("stop_date")),
        }
    }

}
