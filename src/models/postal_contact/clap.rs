use clap::{App, Arg, ArgMatches};
use crate::types;
use crate::models::postal_contact::postal_contact::PostalContact as T;

impl crate::traits::clap_able::ClapAble for T {

    fn init_clap_app<'a, 'ar>(app: App<'a, 'ar>) -> App<'a, 'ar> {
        app
        .arg(Arg::with_name("country")
            .help("country; example: \"US\" is United States")
            .long("country")
            .value_name("TEXT")
            .takes_value(true))
        .arg(Arg::with_name("region")
            .help("region/province/state; example: \"CA\" is California")
            .long("region")
            .value_name("TEXT")
            .takes_value(true))
        .arg(Arg::with_name("locality")
            .help("locality / city / town; example: \"San Francisco\"")
            .long("locality")
            .value_name("TEXT")
            .takes_value(true))
        .arg(Arg::with_name("neighborhood")
            .help("neighborhood / municipality / suburb; example: \"Mission District\"")
            .long("neighborhood")
            .value_name("TEXT")
            .takes_value(true))
        .arg(Arg::with_name("postal_code")
            .help("postal_code / U.S. ZIP code / routing number; example: \"94101\" is San Francisco downtown area")
            .long("postal_code")
            .value_name("CODE")
            .takes_value(true))
        .arg(Arg::with_name("street_address")
            .help("street_address; example: \"123 Main Street\"")
            .long("street_address")
            .value_name("TEXT")
            .takes_value(true))
        .arg(Arg::with_name("premise_address")
            .help("premise address / apartment name / suite number; example: \"Apartment A1\"")
            .long("premise_address")
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

            // Postal-related
            country_text: types::country_text::from_option_str(matches.value_of("country")),
            region_text: types::region_text::from_option_str(matches.value_of("region")),
            locality_text: types::locality_text::from_option_str(matches.value_of("locality")),
            neighborhood_text: types::neighborhood_text::from_option_str(matches.value_of("neighborhood")),
            postal_code_text: types::postal_code_text::from_option_str(matches.value_of("postal_code")),
            street_address_text: types::street_address_text::from_option_str(matches.value_of("street_address")),
            premise_address_text: types::premise_address_text::from_option_str(matches.value_of("premise_address")),
            global_location_number_text: types::global_location_number_text::from_option_str(matches.value_of("global_location_number")),
        }
    }

}
