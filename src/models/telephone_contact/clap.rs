use clap::{App, Arg, ArgMatches};
use crate::types;
use crate::models::telephone_contact::telephone_contact::TelephoneContact as T;


impl T {
    
    fn arg_label<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("label")
            .help("label; example: \"mobile phone\"")
            .long("label")
            .value_name("TEXT")
            .takes_value(true)
    }

    fn arg_number_text<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("number_text")
            .help("number text; example: \"1 (415) 555-0000\"")
            .long("number_text")
            .value_name("TEXT")
            .takes_value(true)
    }

    fn arg_e164<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("e164")
            .help("e164; example: \"14155550000\"")
            .long("e164")
            .value_name("TEXT")
            .takes_value(true)
    }

    fn arg_e164_country_code<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("e164_country_code")
            .help("e164 country code; example: \"1\" is United States")
            .long("e164_country_code")
            .value_name("NUMBER")
            .takes_value(true)
    }

    fn arg_e164_national_destination_code<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("e164_national_destination_code")
            .help("e164 national destination code; example: \"415\" is San Francisco")
            .long("e164_national_destination_code")
            .value_name("NUMBER")
            .takes_value(true)
    }

    fn arg_e164_group_identification_code<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("e164_group_identification_code")
            .help("e164 group identification code; example: TODO")
            .long("e164_group_identification_code")
            .value_name("NUMBER")
            .takes_value(true)
    }

    fn arg_e164_trial_identification_code<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("e164_trial_identification_code")
            .help("e164 trial identification code; example: TODO")
            .long("e164_trial_identification_code")
            .value_name("NUMBER")
            .takes_value(true)
    }

    fn arg_e164_subscriber_number<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("e164_subscriber_number")
            .help("e164 subscriber number; example: \"555-0000\"")
            .long("e164_subscriber_number")
            .value_name("TEXT")
            .takes_value(true)
    }

}

impl crate::traits::clap_able::ClapAble for T {

    fn init_clap_app<'a, 'ar>(app: App<'a, 'ar>) -> App<'a, 'ar> {
        let app = app.arg(Self::arg_label());
        let app = app.arg(Self::arg_number_text());
        let app = app.arg(Self::arg_e164());
        let app = app.arg(Self::arg_e164_country_code());
        let app = app.arg(Self::arg_e164_national_destination_code());
        let app = app.arg(Self::arg_e164_group_identification_code());
        let app = app.arg(Self::arg_e164_trial_identification_code());
        let app = app.arg(Self::arg_e164_subscriber_number());
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

            // Telephone-related
            label: types::label::from_option_str(matches.value_of("label")),
            number_text: types::telephone_number_text::from_option_str(matches.value_of("number_text")),

            // E.164-related
            e164_text: types::telephone_e164_text::from_option_str(matches.value_of("e164")),
            e164_country_code: types::telephone_e164_country_code::from_option_str(matches.value_of("e164_country_code")),
            e164_national_destination_code: types::telephone_e164_national_destination_code::from_option_str(matches.value_of("e164_national_destination_code")),
            e164_group_identification_code: types::telephone_e164_group_identification_code::from_option_str(matches.value_of("e164_group_identification_code")),
            e164_trial_identification_code: types::telephone_e164_trial_identification_code::from_option_str(matches.value_of("e164_trial_identification_code")),
            e164_subscriber_number: types::telephone_e164_subscriber_number::from_option_str(matches.value_of("e164_subscriber_number")),
        }
    }

}
