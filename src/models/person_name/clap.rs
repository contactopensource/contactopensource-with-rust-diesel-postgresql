use clap::{App, Arg, ArgMatches};
use crate::types;
use crate::models::person_name::person_name::PersonName as T;

impl T {


    fn arg_given_name<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("given_name")
            .help("given name; example: \"Alice\"")
            .long("given_name")
            .value_name("NAME")
            .takes_value(true)
    }

    fn arg_given_name_phonetic<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("given_name_phonetic")
            .help("given name phonetic; example: \"Alice\"")
            .long("given_name_phonetic")
            .value_name("NAME")
            .takes_value(true)
    }

    fn arg_middle_name<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("middle_name")
            .help("middle name; example: \"Amy\"")
            .long("middle_name")
            .value_name("NAME")
            .takes_value(true)
    }

    fn arg_middle_name_phonetic<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("middle_name_phonetic")
            .help("middle name phonetic; example: \"Amy\"")
            .long("middle_name_phonetic")
            .value_name("NAME")
            .takes_value(true)
    }

    fn arg_family_name<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("family_name")
            .help("family name; example: \"Adams\"")
            .long("family_name")
            .value_name("NAME")
            .takes_value(true)
    }

    fn arg_family_name_phonetic<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("family_name_phonetic")
            .help("family name phonetic; example: \"Adams\"")
            .long("family_name_phonetic")
            .value_name("NAME")
            .takes_value(true)
    }

    fn arg_legal_name<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("legal_name")
            .help("legal name; example: \"Alice Amy Adams\"")
            .long("legal_name")
            .value_name("NAME")
            .takes_value(true)
    }

    fn arg_legal_name_phonetic<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("legal_name_phonetic")
            .help("legal name; example: \"Alice Amy Adams\"")
            .long("legal_name_phonetic")
            .value_name("NAME")
            .takes_value(true)
    }

    fn arg_prefix_name<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("prefix_name")
            .help("prefix name; example: \"Dr.\"")
            .long("prefix_name")
            .value_name("NAME")
            .takes_value(true)
    }

    fn arg_prefix_name_phonetic<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("prefix_name_phonetic")
            .help("prefix name phonetic; example: \"Dr.\"")
            .long("prefix_name_phonetic")
            .value_name("NAME")
            .takes_value(true)
    }

    fn arg_suffix_name<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("suffix_name")
            .help("suffix name; example: \"Jr.\"")
            .long("suffix_name")
            .value_name("NAME")
            .takes_value(true)
    }

    fn arg_suffix_name_phonetic<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("suffix_name_phonetic")
            .help("suffix name phonetic; example: \"Jr.\"")
            .long("suffix_name_phonetic")
            .value_name("NAME")
            .takes_value(true)
    }

    fn arg_salutation<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("salutation")
            .help("salutation; example: \"Dr. Adams\"")
            .long("salutation")
            .value_name("TEXT")
            .takes_value(true)
    }

    fn arg_salutation_phonetic<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("salutation_phonetic")
            .help("salutation phonetic; example: \"Dr. Adams\"")
            .long("salutation_phonetic")
            .value_name("TEXT")
            .takes_value(true)
    }

    fn arg_addressee<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("addressee")
            .help("addressee; example: \"Dr. Alice Adams Jr.\"")
            .long("addressee")
            .value_name("true")
            .takes_value(true)
    }

    fn arg_addressee_phonetic<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("addressee_phonetic")
            .help("addressee phonetic; example: \"Dr. Alice Adams Jr.\"")
            .long("addressee_phonetic")
            .value_name("true")
            .takes_value(true)
    }

    fn arg_nickname<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("nickname")
            .help("nickname; example: \"Ali\"")
            .long("nickname")
            .value_name("NAME")
            .takes_value(true)
    }

    fn arg_nickname_phonetic<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("nickname_phonetic")
            .help("nickname phonetic; example: \"Ali\"")
            .long("nickname_phonetic")
            .value_name("NAME")
            .takes_value(true)
    }

}

impl crate::traits::clap_able::ClapAble for T {

    fn init_clap_app<'a, 'ar>(app: App<'a, 'ar>) -> App<'a, 'ar> {
        let app = app.arg(Self::arg_given_name());
        let app = app.arg(Self::arg_given_name_phonetic());
        let app = app.arg(Self::arg_middle_name());
        let app = app.arg(Self::arg_middle_name_phonetic());
        let app = app.arg(Self::arg_family_name());
        let app = app.arg(Self::arg_family_name_phonetic());
        let app = app.arg(Self::arg_legal_name());
        let app = app.arg(Self::arg_legal_name_phonetic());
        let app = app.arg(Self::arg_prefix_name());
        let app = app.arg(Self::arg_prefix_name_phonetic());
        let app = app.arg(Self::arg_suffix_name());
        let app = app.arg(Self::arg_suffix_name_phonetic());
        let app = app.arg(Self::arg_salutation());
        let app = app.arg(Self::arg_salutation_phonetic());
        let app = app.arg(Self::arg_addressee());
        let app = app.arg(Self::arg_addressee_phonetic());
        let app = app.arg(Self::arg_nickname());
        let app = app.arg(Self::arg_nickname_phonetic());
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
            given_name: types::person_given_name::from_option_str(matches.value_of("givem_name")),
            given_name_phonetic: types::person_given_name::from_option_str(matches.value_of("givem_name_phonetic")),
            middle_name: types::person_middle_name::from_option_str(matches.value_of("middle_name")),
            middle_name_phonetic: types::person_middle_name::from_option_str(matches.value_of("middle_name_phonetic")),
            family_name: types::person_family_name::from_option_str(matches.value_of("family_name")),
            family_name_phonetic: types::person_family_name::from_option_str(matches.value_of("family_name_phonetic")),
            legal_name: types::person_legal_name::from_option_str(matches.value_of("legal_name")),
            legal_name_phonetic: types::person_legal_name::from_option_str(matches.value_of("legal_name_phonetic")),
            prefix_name: types::person_prefix_name::from_option_str(matches.value_of("prefix_name")),
            prefix_name_phonetic: types::person_prefix_name::from_option_str(matches.value_of("prefix_name_phonetic")),
            suffix_name: types::person_suffix_name::from_option_str(matches.value_of("suffix_name")),
            suffix_name_phonetic: types::person_suffix_name::from_option_str(matches.value_of("suffix_name_phonetic")),
            salutation_name: types::person_salutation_name::from_option_str(matches.value_of("salutation_name")),
            salutation_name_phonetic: types::person_salutation_name::from_option_str(matches.value_of("salutation_name_phonetic")),
            addressee_name: types::person_addressee_name::from_option_str(matches.value_of("addressee_name")),
            addressee_name_phonetic: types::person_addressee_name::from_option_str(matches.value_of("addressee_name_phonetic")),
            nickname: types::person_nickname::from_option_str(matches.value_of("nickname")),
            nickname_phonetic: types::person_nickname::from_option_str(matches.value_of("nickname_phonetic")),

        }
    }

}
