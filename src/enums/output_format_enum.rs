//use ::contactopensource::enums::output_format_enum::OutputFormatEnum;
use ::std::collections::HashMap;

pub type OutputFormatEnumType = usize; //TODO sunset

pub const OUTPUT_FORMAT_ENUM_NONE: OutputFormatEnumType = 0;
pub const OUTPUT_FORMAT_ENUM_TEXT: OutputFormatEnumType = 1;
pub const OUTPUT_FORMAT_ENUM_JSON: OutputFormatEnumType = 2;

// #[derive(Clone, Debug)]
// enum OutputFormatEnum {
//     Text,
//     JSON,
// }

lazy_static! {
    static ref BY_NAME: HashMap<&'static str, OutputFormatEnumType> = {
        hashmap!{
            "text" => OUTPUT_FORMAT_ENUM_TEXT,
            "json" => OUTPUT_FORMAT_ENUM_JSON,
        }
    };
}

lazy_static! {
    static ref TO_NAME: HashMap<OutputFormatEnumType, &'static str, > = {
        hashmap!{
            OUTPUT_FORMAT_ENUM_TEXT => "text",
            OUTPUT_FORMAT_ENUM_JSON => "json",
        }
    };
}
