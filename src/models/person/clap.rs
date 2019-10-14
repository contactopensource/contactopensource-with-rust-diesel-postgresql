use clap::{App, Arg, ArgMatches};
use crate::types;
use crate::models::person::person::Person as T;

impl T {


    fn arg_birth_date<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("birth_date")
            .help("birth date; example: \"2000-01-01\"")
            .long("birth_date")
            .value_name("DATE")
            .takes_value(true)
    }

    fn arg_death_date<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("death_date")
            .help("death date; example: \"2099-01-01\"")
            .long("death_date")
            .value_name("DATE")
            .takes_value(true)
    }

    fn arg_mass_as_grams<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("mass_as_grams")
            .help("contact mass as grams; example: 70000")
            .long("mass_as_grams")
            .value_name("GRAMS")
            .takes_value(true)
    }

    fn arg_height_as_meters<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("height_as_meters")
            .help("contact height as meters; example: 170")
            .long("height_as_meters")
            .value_name("METERS")
            .takes_value(true)
    }

    fn arg_org_name<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("org_name")
            .help("org name; example: \"Acme Company\"")
            .long("org_name")
            .value_name("NAME")
            .takes_value(true)
    }

    fn arg_org_team<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("org_team")
            .help("org team; example: \"Department of Widgets\"")
            .long("org_team")
            .value_name("NAME")
            .takes_value(true)
    }

    fn arg_org_role<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("org_role")
            .help("org role; example: \"Manager of Widgets\"")
            .long("org_role")
            .value_name("NAME")
            .takes_value(true)
    }

}

impl crate::traits::clap_able::ClapAble for T {

    fn init_clap_app<'a, 'ar>(app: App<'a, 'ar>) -> App<'a, 'ar> {
        let app = app.arg(Self::arg_birth_date());
        let app = app.arg(Self::arg_death_date());
        let app = app.arg(Self::arg_mass_as_grams());
        let app = app.arg(Self::arg_height_as_meters());
        let app = app.arg(Self::arg_org_name());
        let app = app.arg(Self::arg_org_team());
        let app = app.arg(Self::arg_org_role());
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
            birth_date: types::date::from_option_str(matches.value_of("birth_date")),
            death_date: types::date::from_option_str(matches.value_of("death_date")),

            // Physical-related
            mass_as_grams: types::grams::from_option_str(matches.value_of("mass_as_grams")),
            height_as_meters: types::meters::from_option_str(matches.value_of("height_as_meters")),

            // Org-related
            org_name: types::text::from_option_str(matches.value_of("org_name")),
            org_team: types::text::from_option_str(matches.value_of("org_team")),
            org_role: types::text::from_option_str(matches.value_of("org_role")),
        }
    }

}
