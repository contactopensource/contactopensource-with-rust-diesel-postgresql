use clap::{App, Arg, ArgMatches};
use crate::types;
use crate::models::geo_code::geo_code::GeoCode as T;

impl crate::traits::clap_able::ClapAble for T {

    fn init_clap_app<'a, 'ar>(app: App<'a, 'ar>) -> App<'a, 'ar> {
        app
        .arg(Arg::with_name("text")
            .help("text; example: \"A1+B2\"; see: https://github.com/google/open-location-code")
            .long("text")
            .value_name("TEXT")
            .takes_value(true))
        .arg(Arg::with_name("coder_id")
            .help("coder id; example: fda15956587d3766862f72fe5ab1feea is https://github.com/google/open-location-code")
            .long("coder_id")
            .value_name("TEXT")
            .takes_value(true))
        .arg(Arg::with_name("latitude")
            .help("latitude; example: '37.8199' is 37.8199° N at the Golden Gate Bridge")
            .long("latitude")
            .value_name("NUMBER")
            .takes_value(true))
        .arg(Arg::with_name("longitude")
            .help("longitude; example: '122.4783' is 122.4783° W at the Golden Gate Bridge")
            .long("longitude")
            .value_name("NUMBER")
            .takes_value(true))
        .arg(Arg::with_name("altitude")
            .help("altitude; example: '67.056' is 67.056 meters to local surface of the earth")
            .long("altitude")
            .value_name("NUMBER")
            .takes_value(true))
        .arg(Arg::with_name("elevation")
            .help("elevation; example: '67.056' is 67.056 meters to global sea level")
            .long("elevation")
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

            // Code-related
            coder_id: types::id::from_option_str(matches.value_of("coder_id")),
            text: types::text::from_option_str(matches.value_of("text")),
            latitude: types::latitude::from_option_str(matches.value_of("latitude")),
            longitude: types::longitude::from_option_str(matches.value_of("longitude")),
            altitude: types::altitude::from_option_str(matches.value_of("altitude")),
            elevation: types::elevation::from_option_str(matches.value_of("elevation")),

        }
    }

}