use clap::{App, Arg, ArgMatches};
use crate::types;
use crate::models::media_type::media_type::MediaType as T;

impl crate::traits::clap_able::ClapAble for T {

    fn init_clap_app<'a, 'ar>(app: App<'a, 'ar>) -> App<'a, 'ar> {
        app
        .arg(Arg::with_name("text")
            .help("text; example: \"text/plain\"")
            .long("text")
            .value_name("TEXT")
            .takes_value(true))
        .arg(Arg::with_name("supertype")
            .help("supertype; example: \"text\"")
            .long("supertype")
            .value_name("TEXT")
            .takes_value(true))
        .arg(Arg::with_name("tree")
            .help("tree; example: \"x.\"")
            .long("tree")
            .value_name("TEXT")
            .takes_value(true))
        .arg(Arg::with_name("subtype")
            .help("subtype; example: \"plain\"")
            .long("subtype")
            .value_name("TEXT")
            .takes_value(true))
        .arg(Arg::with_name("suffix")
            .help("tree; example: \"+json\"")
            .long("suffix")
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

            // Content-related
            text: types::media_type_text::from_option_str(matches.value_of("text")),
            supertype: types::media_type_supertype::from_option_str(matches.value_of("supertype")),
            subtype: types::media_type_subtype::from_option_str(matches.value_of("subtype")),
            tree: types::media_type_tree::from_option_str(matches.value_of("tree")),
            suffix: types::media_type_suffix::from_option_str(matches.value_of("suffix")),
            parameters: types::media_type_parameters::from_option_str(matches.value_of("suffix")),
        }
    }

}
