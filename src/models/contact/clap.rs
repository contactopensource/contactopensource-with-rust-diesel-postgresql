use clap::{App, Arg, ArgMatches};
use crate::types;
use crate::models::contact::contact::Contact as T;

impl crate::traits::clap_able::ClapAble for T {

    fn init_clap_app<'a, 'ar>(app: App<'a, 'ar>) -> App<'a, 'ar> {
        app
        .arg(Arg::with_name("name")
            .help("name; typically freeform full name; example: \"Alice Adams\"")
            .long("name")
            .value_name("TEXT")
            .takes_value(true))
        .arg(Arg::with_name("emoji")
            .help("emoji; typically emoji characters; example \"U+1F60A\" is smiling face with smiling eyes")
            .long("emoji")
            .value_name("TEXT")
            .takes_value(true))
        .arg(Arg::with_name("note")
            .help("note, typically freeform; example: \"Alice is a good friend, nearby neighbor, and fast runner\"")
            .long("note")
            .value_name("TEXT")
            .takes_value(true))
        .arg(Arg::with_name("image_uri")
            .help("image URI; example: \"https://example.com/image.jpg\"")
            .long("image_uri")
            .value_name("URI")
            .takes_value(true))
        .arg(Arg::with_name("color_hex")
            .help("color hex six digit code; example: \"FF0000\" is red")
            .long("HEX")
            .value_name("color_hex")
            .takes_value(true))
        .arg(Arg::with_name("css_class")
            .help("CSS class, as a space-separated list; example: \"friend neighbor runner\" is a cascading style sheet class")
            .long("CLASS")
            .value_name("css_class")
            .takes_value(true))
        .arg(Arg::with_name("star_count")
            .help("star count; example: '5' means 5-star rating")
            .long("INT")
            .value_name("star_count")
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

            // General-related
            name: types::text::from_option_str(matches.value_of("name")),
            emoji: types::text::from_option_str(matches.value_of("emoji")),

            // Display-related
            image_uri: types::uri_string::from_option_str(matches.value_of("image_uri")),
            color_hex: types::color_hex_upper_string::from_option_str(matches.value_of("color_hex")),
            css_class: types::css_class_name::from_option_str(matches.value_of("css_class")),
            star_count: types::star_count::from_option_str(matches.value_of("star_count")),
        }
    }

}