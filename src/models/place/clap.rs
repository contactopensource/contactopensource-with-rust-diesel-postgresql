use clap::{App, Arg, ArgMatches};
use crate::types;
use crate::models::place::place::Place as T;

impl crate::traits::clap_able::ClapAble for T {

    fn init_clap_app<'a, 'ar>(app: App<'a, 'ar>) -> App<'a, 'ar> {
        app
        // Name-related
        .arg(Arg::with_name("name")
            .help("name; example: \"Lunch\"")
            .long("name")
            .value_name("NAME")
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

            // Name-related
            name: types::text::from_option_str(matches.value_of("name")),
        }
    }

}
