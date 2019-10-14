use clap::{App, Arg, ArgMatches};
use crate::types;
use crate::models::email_contact::email_contact::EmailContact as T;

impl T {

    fn arg_address<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("address")
            .help("address; example: \"Alice Adams <alice.anderson@example.com>\"")
            .long("address")
            .value_name("TEXT")
            .takes_value(true)
    }

    fn arg_display_name<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("display_name")
            .help("display name; example: \"Alice Adams\"")
            .long("display_name")
            .value_name("TEXT")
            .takes_value(true)
    }

    fn arg_addr_spec<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("addr_spec")
            .help("address specification; example: \"alice.anderson@example.com\"")
            .long("addr_spec")
            .value_name("TEXT")
            .takes_value(true)
    }

}

impl crate::traits::clap_able::ClapAble for T {

    fn init_clap_app<'a, 'ar>(app: App<'a, 'ar>) -> App<'a, 'ar> {
        let app = app.arg(Self::arg_address());
        let app = app.arg(Self::arg_display_name());
        let app = app.arg(Self::arg_addr_spec());
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

            // Email-related
            address: types::email_address::from_option_str(matches.value_of("address")),
            display_name: types::email_display_name::from_option_str(matches.value_of("display_name")),
            addr_spec: types::email_addr_spec::from_option_str(matches.value_of("addr_spec")),
        }
    }

}
