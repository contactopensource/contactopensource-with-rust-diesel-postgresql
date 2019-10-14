use clap::{App, Arg, ArgMatches};
use crate::types;
use crate::models::locale::locale::Locale as T;


impl T {

    fn arg_text<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("text")
            .help("text; example: \"en-US\"")
            .long("text")
            .value_name("TEXT")
            .takes_value(true)
    }

    fn arg_language_code<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("language_code")
            .help("language code; example: \"en\" is English")
            .long("language_code")
            .value_name("CODE")
            .takes_value(true)
    }

    fn arg_country_code<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("country_code")
            .help("country code; example: \"US\" is United States")
            .long("country_code")
            .value_name("CODE")
            .takes_value(true)
    }

    fn arg_script_code<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("script_code")
            .help("script code; example: \"Latn\" is Latin")
            .long("script_code")
            .value_name("CODE")
            .takes_value(true)
    }

    fn arg_region_code<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("region_code")
            .help("region code; example: \"QO\" is Outlying Oceana")
            .long("region_code")
            .value_name("CODE")
            .takes_value(true)
    }

    fn arg_decimal_separator<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("decimal_separator")
            .help("decimal separator; example: \".\" is for United States")
            .long("decimal_separator")
            .value_name("SEPARATOR")
            .takes_value(true)
    }

    fn arg_grouping_separator<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("grouping_separator")
            .help("grouping code; example: \",\" is for United States")
            .long("grouping_separator")
            .value_name("SEPARATOR")
            .takes_value(true)
    }

    fn arg_currency_code<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("currency_code")
            .help("currency code; example: \"USD\" is United States Dollar")
            .long("currency_code")
            .value_name("CODE")
            .takes_value(true)
    }

    fn arg_currency_symbol<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("currency_symbol")
            .help("currency symbol; example: $ is United States Dollar")
            .long("currency_symbol")
            .value_name("SYMBOL")
            .takes_value(true)
    }

    fn arg_quotation_start_delimiter<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("quotation_start_delimiter")
            .help("quotation start delimiter; “ (U+201C) is left double quotation mark")
            .long("quotation_start_delimiter")
            .value_name("DELIMITER")
            .takes_value(true)
    }

    fn arg_quotation_stop_delimiter<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("quotation_stop_delimiter")
            .help("quotation stop delimiter; example: ” (U+201D) is right double quotation mark")
            .long("quotation_stop_delimiter")
            .value_name("DELIMITER")
            .takes_value(true)
    }

}

impl crate::traits::clap_able::ClapAble for T {

    fn init_clap_app<'a, 'ar>(app: App<'a, 'ar>) -> App<'a, 'ar> {
        let app = app.arg(Self::arg_text());
        let app = app.arg(Self::arg_language_code());
        let app = app.arg(Self::arg_country_code());
        let app = app.arg(Self::arg_script_code());
        let app = app.arg(Self::arg_region_code());
        let app = app.arg(Self::arg_decimal_separator());
        let app = app.arg(Self::arg_grouping_separator());
        let app = app.arg(Self::arg_currency_code());
        let app = app.arg(Self::arg_currency_symbol());
        let app = app.arg(Self::arg_quotation_start_delimiter());
        let app = app.arg(Self::arg_quotation_stop_delimiter());
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

            // Code-related
            text: types::locale_string::from_option_str(matches.value_of("text")),
            language_code: types::locale_language_code::from_option_str(matches.value_of("language_code")),
            country_code: types::locale_country_code::from_option_str(matches.value_of("country_code")),
            script_code: types::locale_script_code::from_option_str(matches.value_of("script_code")),
            region_code: types::locale_region_code::from_option_str(matches.value_of("region_code")),
            variant_code: types::locale_variant_code::from_option_str(matches.value_of("variant_code")),

            // Representation-related
            decimal_separator: types::locale_decimal_separator::from_option_str(matches.value_of("decimal_separator")),
            grouping_separator: types::locale_grouping_separator::from_option_str(matches.value_of("grouping_separator")),
            currency_code: types::currency_code::from_option_str(matches.value_of("currency_code")),
            currency_symbol: types::currency_symbol::from_option_str(matches.value_of("currency_symbol")),
            quotation_start_delimiter: types::quotation_start_delimiter::from_option_str(matches.value_of("quotation_start_delimiter")),
            quotation_stop_delimiter: types::quotation_stop_delimiter::from_option_str(matches.value_of("quotation_stop_delimiter")),
        }
    }

}
