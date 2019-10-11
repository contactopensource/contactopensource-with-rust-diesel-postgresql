pub use std::collections::HashSet;
pub use ::rand::{Rng, thread_rng, seq::SliceRandom};

// TODO
pub mod id;
pub mod css_class_name;
pub mod color_hex_upper_string;
pub mod star_count;
pub mod user_name;
pub mod field_name;
pub mod number;

// Std
pub mod count;
pub mod hex_upper_string;
pub mod label;
pub mod state;
pub mod text;
pub mod date;
pub mod timestamp;
pub mod typecast;
pub mod probability;
pub mod weight;

// Content
pub mod html_string;
pub mod json_string;
pub mod json_value;
pub mod markdown_string;
pub mod xml_string;
pub mod yaml_string;

// Currency
pub mod currency_code;
pub mod currency_name;
pub mod currency_symbol;

// Email
pub mod email_addr_spec;
pub mod email_address;
pub mod email_display_name;

// Database
pub mod db_database_name;
pub mod db_schema_name;
pub mod db_table_name;
pub mod db_column_name;
pub mod db_row_id;

// Fab
pub mod alpha_word;
pub mod animal_word;
pub mod fruit_word;

// Geo
pub mod latitude;
pub mod longitude;
pub mod altitude;
pub mod elevation;

// Industry
pub mod global_location_number_text;
pub mod isic_v4_code;
pub mod isic_v4_name;
pub mod legal_entity_identifier;
pub mod ticker_symbol;
pub mod value_added_tax_identification_number;

// Locale
pub mod locale_string;
pub mod locale_language_code;
pub mod locale_country_code;
pub mod locale_script_code;
pub mod locale_region_code;
pub mod locale_variant_code;
pub mod locale_decimal_separator;
pub mod locale_grouping_separator;
pub mod quotation_start_delimiter;
pub mod quotation_stop_delimiter;

// Metrics
pub mod amperes;
pub mod candelas;
pub mod grams;
pub mod joules;
pub mod kelvins;
pub mod liters;
pub mod meters;
pub mod meters_2;
pub mod meters_3;
pub mod meters_per_second;
pub mod metrics;
pub mod moles;
pub mod ohms;
pub mod seconds;
pub mod watts;

// Media type
pub mod media_type_supertype;
pub mod media_type_subtype;
pub mod media_type_suffix;
pub mod media_type_text;
pub mod media_type_tree;
pub mod media_type_parameter;
pub mod media_type_parameters;

// Net
pub mod host_name;
pub mod node_name;
pub mod domain_name;
pub mod tld_name;
pub mod uri_scheme;
pub mod uri_string;

// Passport
pub mod passport_country_text;
pub mod passport_number_text;

// Path
pub mod path_name;
pub mod path_dir_name;
pub mod path_base_name;
pub mod path_ext_name;

// Personal name
pub mod person_name;
pub mod person_given_name;
pub mod person_middle_name;
pub mod person_family_name;
pub mod person_legal_name;
pub mod person_prefix_name;
pub mod person_suffix_name;
pub mod person_salutation_name;
pub mod person_addressee_name;
pub mod person_nickname;

// Personal pronoun
pub mod subject_pronoun;
pub mod object_pronoun;
pub mod dependent_possessive_pronoun;
pub mod independent_possessive_pronoun;
pub mod reflexive_pronoun;
pub mod intensive_pronoun;
pub mod disjunctive_pronoun;

// Postal
pub mod country_text;
pub mod region_text;
pub mod locality_text;
pub mod neighborhood_text;
pub mod postal_code_text;
pub mod street_address_text;
pub mod premise_address_text;

// Telephone
pub mod telephone_label;
pub mod telephone_number_text;
pub mod telephone_e164_country_code;
pub mod telephone_e164_group_identification_code;
pub mod telephone_e164_national_destination_code;
pub mod telephone_e164_subscriber_number;
pub mod telephone_e164_text;
pub mod telephone_e164_trial_identification_code;



