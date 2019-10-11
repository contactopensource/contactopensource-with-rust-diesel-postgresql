use crate::helpers::parse;

pub fn to_country_text(s: Option<&str>) -> Option<String> { parse::a::option_str_to_option_string(s) }
pub fn to_region_text(s: Option<&str>) -> Option<String> { parse::a::option_str_to_option_string(s) }
pub fn to_locality_text(s: Option<&str>) -> Option<String> { parse::a::option_str_to_option_string(s) }
pub fn to_neighborhood_text(s: Option<&str>) -> Option<String> { parse::a::option_str_to_option_string(s) }
pub fn to_postal_code_text(s: Option<&str>) -> Option<String> { parse::a::option_str_to_option_string(s) }
pub fn to_street_address_text(s: Option<&str>) -> Option<String> { parse::a::option_str_to_option_string(s) }
pub fn to_premise_address_text(s: Option<&str>) -> Option<String> { parse::a::option_str_to_option_string(s) }
pub fn to_global_location_number_text(s: Option<&str>) -> Option<String> { parse::a::option_str_to_option_string(s) }
