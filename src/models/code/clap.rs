use clap::{App, Arg, ArgMatches};
use crate::types;
use crate::models::code::code::Code as T;

impl crate::traits::clap_able::ClapAble for T {

    fn init_clap_app<'a, 'ar>(app: App<'a, 'ar>) -> App<'a, 'ar> {
        app
        .arg(Arg::with_name("set_id")
            .help("set id; example: 91d0c771f4e98664bc980f48a90d535e means ISO")
            .long("set_id")
            .value_name("ID")
            .takes_value(true))
        .arg(Arg::with_name("text")
            .help("text; example: \"A\"")
            .long("text")
            .value_name("TEXT")
            .takes_value(true))
        .arg(Arg::with_name("name")
            .help("text; example: \"Agricultural\"")
            .long("name")
            .value_name("TEXT")
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

            // Link-related
            set_id: types::id::from_option_str(matches.value_of("set_id")),
            text: types::text::from_option_str(matches.value_of("text")),
            name: types::text::from_option_str(matches.value_of("name")),
        }
    }

}