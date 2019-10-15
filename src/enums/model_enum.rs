//use ::contactopensource::enums::model_enum::ModelEnum;
//use crate::models;
use ::std::collections::HashMap;

pub type ModelEnumType = usize;

pub const MODEL_ENUM_NONE: ModelEnumType = 0;
pub const MODEL_ENUM_ITEM: ModelEnumType = 1;
pub const MODEL_ENUM_ARC:  ModelEnumType = 2;

// #[derive(Clone, Debug)]
// enum ModelEnum {
//     Item,
//     Arc,
// }

lazy_static! {
    static ref BY_NAME: HashMap<&'static str, ModelEnumType> = {
        hashmap!{
            "count" => MODEL_ENUM_ITEM,
            "list" => MODEL_ENUM_ARC,
        }
    };
}

lazy_static! {
    static ref TO_NAME: HashMap<ModelEnumType, &'static str> = {
        hashmap!{
            MODEL_ENUM_ITEM => "item",
            MODEL_ENUM_ARC => "arc",
        }
    };
}
