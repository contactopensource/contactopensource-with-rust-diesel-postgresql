use ::clap::{App, ArgMatches};

pub trait ClapAble {
    fn init_clap_app<'a, 'ar>(app: App<'a, 'ar>) -> App<'a, 'ar>;
    fn from_clap_arg_matches(matches: &ArgMatches) -> Self;
}