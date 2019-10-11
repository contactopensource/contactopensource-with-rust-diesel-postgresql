use clap::{App, Arg, ArgMatches};
use crate::types;
use crate::models::passport_contact::passport_contact::PassportContact as T;

impl crate::traits::clap_able::ClapAble for T {

    fn init_clap_app<'a, 'ar>(app: App<'a, 'ar>) -> App<'a, 'ar> {
        app
        .arg(Arg::with_name("country")
            .help("country text; example: \"US\" is United States")
            .long("country")
            .value_name("TEXT")
            .takes_value(true))
        .arg(Arg::with_name("number")
            .help("number text; example: \"0000-0000-0000-0000\"")
            .long("number")
            .value_name("TEXT")
            .takes_value(true))
        .arg(Arg::with_name("start_date")
            .help("start date; example: \"2020-01-01\"")
            .long("start_date")
            .value_name("DATE")
            .takes_value(true))
        .arg(Arg::with_name("stop_date")
            .help("stop date; example: \"2030-01-01\"")
            .long("stop_date")
            .value_name("DATE")
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

            // Place-related
            country_text: types::passport_country_text::from_option_str(matches.value_of("country_text")),
            number_text: types::passport_number_text::from_option_str(matches.value_of("number_text")),

            // Time-related
            valid_start_date: types::date::from_option_str(matches.value_of("start_date")),
            valid_stop_date: types::date::from_option_str(matches.value_of("stop_date")),
        }
    }

}
