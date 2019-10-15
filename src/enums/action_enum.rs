//use ::contactopensource::enums::action_enum::ActionEnum;
use ::std::collections::HashMap;

pub type ActionEnumType = usize; //TODO sunset

pub const ACTION_ENUM_NONE:    ActionEnumType = 0;
pub const ACTION_ENUM_COUNT:   ActionEnumType = 1;
pub const ACTION_ENUM_LIST:    ActionEnumType = 2;
pub const ACTION_ENUM_CREATE:  ActionEnumType = 3;
pub const ACTION_ENUM_READ:    ActionEnumType = 4;
pub const ACTION_ENUM_UPDATE:  ActionEnumType = 5;
pub const ACTION_ENUM_DELETE:  ActionEnumType = 6;

// #[derive(Clone, Debug)]
// enum ActionEnum {
//     Count,
//     List,
//     Create,
//     Read,
//     Update,
//     Delete,
// }

lazy_static! {
    static ref BY_NAME: HashMap<&'static str, ActionEnumType> = {
        hashmap!{
            "count" => ACTION_ENUM_COUNT,
            "list" => ACTION_ENUM_LIST,
            "create" => ACTION_ENUM_CREATE,
            "read" => ACTION_ENUM_READ,
            "update" => ACTION_ENUM_UPDATE,
            "delete" => ACTION_ENUM_DELETE,
        }
    };
}

lazy_static! {
    static ref TO_NAME: HashMap<ActionEnumType, &'static str> = {
        hashmap!{
            ACTION_ENUM_COUNT => "count",
            ACTION_ENUM_LIST => "list",
            ACTION_ENUM_CREATE => "create",
            ACTION_ENUM_READ => "read",
            ACTION_ENUM_UPDATE => "update",
            ACTION_ENUM_DELETE => "delete",
        }
    };
}
