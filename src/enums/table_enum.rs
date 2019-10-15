//use ::contactopensource::enums::table_enum::TableEnum;
//use crate::schema;
use ::std::collections::HashMap;

pub type TableEnumType = usize; //TODO sunset

pub const TABLE_ENUM_NONE:  TableEnumType = 0;
pub const TABLE_ENUM_ITEMS: TableEnumType = 1;
pub const TABLE_ENUM_ARCS:  TableEnumType = 2;

// #[derive(Clone, Debug)]
// enum TableEnum {
//     Items,
//     Arcs,
// }

lazy_static! {
    static ref BY_NAME: HashMap<&'static str, TableEnumType> = {
        hashmap!{
            "items" => TABLE_ENUM_ITEMS,
            "arcs" => TABLE_ENUM_ARCS,
        }
    };
}

lazy_static! {
    static ref TO_NAME: HashMap<TableEnumType, &'static str> = {
        hashmap!{
            TABLE_ENUM_ITEMS => "items",
            TABLE_ENUM_ARCS => "arcs",
        } 
    };
}
