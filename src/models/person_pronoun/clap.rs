use clap::{App, Arg, ArgMatches};
use crate::types;
use crate::models::person_pronoun::person_pronoun::PersonPronoun as T;

impl crate::traits::clap_able::ClapAble for T {

    fn init_clap_app<'a, 'ar>(app: App<'a, 'ar>) -> App<'a, 'ar> {
        app
        .arg(Arg::with_name("subject pronoun")
            .help("subject pronoun; example: \"she\"")
            .long("subject_pronoun")
            .value_name("PRONOUN")
            .takes_value(true))
        .arg(Arg::with_name("object_pronoun")
            .help("object pronoun; example: \"her\"")
            .long("object_pronoun")
            .value_name("PRONOUN")
            .takes_value(true))
        .arg(Arg::with_name("dependent_possessive_pronoun")
            .help("dependent possessive pronoun; example: \"her\"")
            .long("dependent_possessive_pronoun")
            .value_name("PRONOUN")
            .takes_value(true))
        .arg(Arg::with_name("independent_possessive_pronoun")
            .help("independent possessive pronoun; example: \"hers\"")
            .long("independent_possessive_pronoun")
            .value_name("PRONOUN")
            .takes_value(true))
        .arg(Arg::with_name("reflexive_pronoun")
            .help("reflexive pronoun; example: \"herself\"")
            .long("reflexive_pronoun")
            .value_name("PRONOUN")
            .takes_value(true))
        .arg(Arg::with_name("intensive_pronoun")
            .help("intensive pronoun; example: \"herself\"")
            .long("intensive_pronoun")
            .value_name("PRONOUN")
            .takes_value(true))
        .arg(Arg::with_name("disjunctive_pronoun")
            .help("disjunctive pronoun; example: \"her\"")
            .long("disjunctive_pronoun")
            .value_name("PRONOUN")
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

            // Pronoun-related
            subject_pronoun: types::subject_pronoun::from_option_str(matches.value_of("subject_pronoun")),
            object_pronoun: types::object_pronoun::from_option_str(matches.value_of("object_pronoun")),
            dependent_possessive_pronoun: types::dependent_possessive_pronoun::from_option_str(matches.value_of("dependent_possessive_pronoun")),
            independent_possessive_pronoun: types::independent_possessive_pronoun::from_option_str(matches.value_of("independent_possessive_pronoun")),
            reflexive_pronoun: types::reflexive_pronoun::from_option_str(matches.value_of("reflexive_pronoun")),
            intensive_pronoun: types::intensive_pronoun::from_option_str(matches.value_of("intensive_pronoun")),
            disjunctive_pronoun: types::disjunctive_pronoun::from_option_str(matches.value_of("disjunctive_pronoun")),

        }
    }

}
