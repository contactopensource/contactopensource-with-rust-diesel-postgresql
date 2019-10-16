//! Test insert with Diesel

#[cfg(test)]
mod test_database_with_diesel {

    extern crate contactopensource;

    use ::diesel::pg::Pg;

    use ::contactopensource::traits::{as_serde_json_value::*, as_sql_insert::*, fab_able::*};

    ////
    //
    // Helpers
    //
    ////

    // fn model_to_json_value<T: AsSerdeJsonValue>(x: T) -> t::json_value::JSONValue {
    //     x.as_serde_json_value()
    // }

    // fn model_to_insertable_struct<T: AsSerdeJsonValue>(x: T) -> T {
    //     serde_json::from_value::<T>(x.as_serde_json_value()).unwrap()
    // }

    ////
    //
    // Smoke tests
    //
    ////

// macro_rules! db_create {
//     ($name:?, $Insertable:ty) => {
//         #[test]
//         fn test_insert_$name_as_insertable_struct_test_sql() {
//             use ::contactopensource::schema::items::dsl::*;
//             let x = ::contactopensource::models::$name::fab::fab();
//             let json_value = x.as_serde_json_value();
//             let insertable_struct: Item = serde_json::from_value::<Item>(json_value).unwrap();
//             let query = diesel::insert_into(items).values(&insertable_struct);
//             let exp = x.as_sql_insert();
//             let act = diesel::debug_query::<Pg, _>(&query).to_string();
//             assert_eq!(exp, act);
//         }
//     }
// }

    //// Item
    #[test]
    fn test_insert_item_as_insertable_struct_test_sql() {
        use ::contactopensource::schema::items::table as my_table;
        use ::contactopensource::models::item::item::Item as MyModel;
        let x: MyModel = MyModel::fab();
        let json_value = x.as_serde_json_value();
        let insertable_struct: MyModel = serde_json::from_value::<MyModel>(json_value).unwrap();
        let query = diesel::insert_into(my_table).values(&insertable_struct);
        let exp = x.as_sql_insert();
        let act = diesel::debug_query::<Pg, _>(&query).to_string();
        assert_eq!(exp, act);
    }

    //// Arc
    #[test]
    fn test_insert_arc_as_insertable_struct_test_sql() {
        use ::contactopensource::schema::arcs::table as my_table;
        use ::contactopensource::models::arc::arc::Arc as MyModel;
        let x: MyModel = MyModel::fab();
        let json_value = x.as_serde_json_value();
        let insertable_struct: MyModel = serde_json::from_value::<MyModel>(json_value).unwrap();
        let query = diesel::insert_into(my_table).values(&insertable_struct);
        let exp = x.as_sql_insert();
        let act = diesel::debug_query::<Pg, _>(&query).to_string();
        assert_eq!(exp, act);
    }

    //// Org
    #[test]
    fn test_insert_org_as_insertable_struct_test_sql() {
        use ::contactopensource::schema::orgs::table as my_table;
        use ::contactopensource::models::org::org::Org as MyModel;
        let x: MyModel = MyModel::fab();
        let json_value = x.as_serde_json_value();
        let insertable_struct: MyModel = serde_json::from_value::<MyModel>(json_value).unwrap();
        let query = diesel::insert_into(my_table).values(&insertable_struct);
        let exp = x.as_sql_insert();
        let act = diesel::debug_query::<Pg, _>(&query).to_string();
        assert_eq!(exp, act);
    }

    //// Person
    #[test]
    fn test_insert_person_as_insertable_struct_test_sql() {
        use ::contactopensource::schema::persons::table as my_table;
        use ::contactopensource::models::person::person::Person as MyModel;
        let x: MyModel = MyModel::fab();
        let json_value = x.as_serde_json_value();
        let insertable_struct: MyModel = serde_json::from_value::<MyModel>(json_value).unwrap();
        let query = diesel::insert_into(my_table).values(&insertable_struct);
        let exp = x.as_sql_insert();
        let act = diesel::debug_query::<Pg, _>(&query).to_string();
        assert_eq!(exp, act);
    }

    //// Place
    #[test]
    fn test_insert_place_as_insertable_struct_test_sql() {
        use ::contactopensource::schema::places::table as my_table;
        use ::contactopensource::models::place::place::Place as MyModel;
        let x: MyModel = MyModel::fab();
        let json_value = x.as_serde_json_value();
        let insertable_struct: MyModel = serde_json::from_value::<MyModel>(json_value).unwrap();
        let query = diesel::insert_into(my_table).values(&insertable_struct);
        let exp = x.as_sql_insert();
        let act = diesel::debug_query::<Pg, _>(&query).to_string();
        assert_eq!(exp, act);
    }

    //// Thing
    #[test]
    fn test_insert_thing_as_insertable_struct_test_sql() {
        use ::contactopensource::schema::things::table as my_table;
        use ::contactopensource::models::thing::thing::Thing as MyModel;
        let x: MyModel = MyModel::fab();
        let json_value = x.as_serde_json_value();
        let insertable_struct: MyModel = serde_json::from_value::<MyModel>(json_value).unwrap();
        let query = diesel::insert_into(my_table).values(&insertable_struct);
        let exp = x.as_sql_insert();
        let act = diesel::debug_query::<Pg, _>(&query).to_string();
        assert_eq!(exp, act);
    }

    //// Event
    #[test]
    fn test_insert_event_as_insertable_struct_test_sql() {
        use ::contactopensource::schema::events::table as my_table;
        use ::contactopensource::models::event::event::Event as MyModel;
        let x: MyModel = MyModel::fab();
        let json_value = x.as_serde_json_value();
        let insertable_struct: MyModel = serde_json::from_value::<MyModel>(json_value).unwrap();
        let query = diesel::insert_into(my_table).values(&insertable_struct);
        let exp = x.as_sql_insert();
        let act = diesel::debug_query::<Pg, _>(&query).to_string();
        assert_eq!(exp, act);
    }

    //// LinkContact
    #[test]
    fn test_insert_link_contact_as_insertable_struct_test_sql() {
        use ::contactopensource::schema::link_contacts::table as my_table;
        use ::contactopensource::models::link_contact::link_contact::LinkContact as MyModel;
        let x: MyModel = MyModel::fab();
        let json_value = x.as_serde_json_value();
        let insertable_struct: MyModel = serde_json::from_value::<MyModel>(json_value).unwrap();
        let query = diesel::insert_into(my_table).values(&insertable_struct);
        let exp = x.as_sql_insert();
        let act = diesel::debug_query::<Pg, _>(&query).to_string();
        assert_eq!(exp, act);
    }

    //// EmailContact
    #[test]
    fn test_insert_email_contact_as_insertable_struct_test_sql() {
        use ::contactopensource::schema::email_contacts::table as my_table;
        use ::contactopensource::models::email_contact::email_contact::EmailContact as MyModel;
        let x: MyModel = MyModel::fab();
        let json_value = x.as_serde_json_value();
        let insertable_struct: MyModel = serde_json::from_value::<MyModel>(json_value).unwrap();
        let query = diesel::insert_into(my_table).values(&insertable_struct);
        let exp = x.as_sql_insert();
        let act = diesel::debug_query::<Pg, _>(&query).to_string();
        assert_eq!(exp, act);
    }

    //// TelephoneContact
    #[test]
    fn test_insert_telephone_contact_as_insertable_struct_test_sql() {
        use ::contactopensource::schema::telephone_contacts::table as my_table;
        use ::contactopensource::models::telephone_contact::telephone_contact::TelephoneContact as MyModel;
        let x: MyModel = MyModel::fab();
        let json_value = x.as_serde_json_value();
        let insertable_struct: MyModel = serde_json::from_value::<MyModel>(json_value).unwrap();
        let query = diesel::insert_into(my_table).values(&insertable_struct);
        let exp = x.as_sql_insert();
        let act = diesel::debug_query::<Pg, _>(&query).to_string();
        assert_eq!(exp, act);
    }

    //// PostalContact
    #[test]
    fn test_insert_postal_contact_as_insertable_struct_test_sql() {
        use ::contactopensource::schema::postal_contacts::table as my_table;
        use ::contactopensource::models::postal_contact::postal_contact::PostalContact as MyModel;
        let x: MyModel = MyModel::fab();
        let json_value = x.as_serde_json_value();
        let insertable_struct: MyModel = serde_json::from_value::<MyModel>(json_value).unwrap();
        let query = diesel::insert_into(my_table).values(&insertable_struct);
        let exp = x.as_sql_insert();
        let act = diesel::debug_query::<Pg, _>(&query).to_string();
        assert_eq!(exp, act);
    }

    //// PassportContact
    #[test]
    fn test_insert_passport_contact_as_insertable_struct_test_sql() {
        use ::contactopensource::schema::passport_contacts::table as my_table;
        use ::contactopensource::models::passport_contact::passport_contact::PassportContact as MyModel;
        let x: MyModel = MyModel::fab();
        let json_value = x.as_serde_json_value();
        let insertable_struct: MyModel = serde_json::from_value::<MyModel>(json_value).unwrap();
        let query = diesel::insert_into(my_table).values(&insertable_struct);
        let exp = x.as_sql_insert();
        let act = diesel::debug_query::<Pg, _>(&query).to_string();
        assert_eq!(exp, act);
    }

    //// Contact
    #[test]
    fn test_insert_contact_as_insertable_struct_test_sql() {
        use ::contactopensource::schema::contacts::table as my_table;
        use ::contactopensource::models::contact::contact::Contact as MyModel;
        let x: MyModel = MyModel::fab();
        let json_value = x.as_serde_json_value();
        let insertable_struct: MyModel = serde_json::from_value::<MyModel>(json_value).unwrap();
        let query = diesel::insert_into(my_table).values(&insertable_struct);
        let exp = x.as_sql_insert();
        let act = diesel::debug_query::<Pg, _>(&query).to_string();
        assert_eq!(exp, act);
    }

    //// Tag
    #[test]
    fn test_insert_tag_as_insertable_struct_test_sql() {
        use ::contactopensource::schema::tags::table as my_table;
        use ::contactopensource::models::tag::tag::Tag as MyModel;
        let x: MyModel = MyModel::fab();
        let json_value = x.as_serde_json_value();
        let insertable_struct: MyModel = serde_json::from_value::<MyModel>(json_value).unwrap();
        let query = diesel::insert_into(my_table).values(&insertable_struct);
        let exp = x.as_sql_insert();
        let act = diesel::debug_query::<Pg, _>(&query).to_string();
        assert_eq!(exp, act);
    }

    //// Code
    #[test]
    fn test_insert_code_as_insertable_struct_test_sql() {
        use ::contactopensource::schema::codes::table as my_table;
        use ::contactopensource::models::code::code::Code as MyModel;
        let x: MyModel = MyModel::fab();
        let json_value = x.as_serde_json_value();
        let insertable_struct: MyModel = serde_json::from_value::<MyModel>(json_value).unwrap();
        let query = diesel::insert_into(my_table).values(&insertable_struct);
        let exp = x.as_sql_insert();
        let act = diesel::debug_query::<Pg, _>(&query).to_string();
        assert_eq!(exp, act);
    }

    //// Locale
    #[test]
    fn test_insert_locale_as_insertable_struct_test_sql() {
        use ::contactopensource::schema::locales::table as my_table;
        use ::contactopensource::models::locale::locale::Locale as MyModel;
        let x: MyModel = MyModel::fab();
        let json_value = x.as_serde_json_value();
        let insertable_struct: MyModel = serde_json::from_value::<MyModel>(json_value).unwrap();
        let query = diesel::insert_into(my_table).values(&insertable_struct);
        let exp = x.as_sql_insert();
        let act = diesel::debug_query::<Pg, _>(&query).to_string();
        assert_eq!(exp, act);
    }

    //// MediaType
    #[test]
    fn test_insert_media_type_as_insertable_struct_test_sql() {
        use ::contactopensource::schema::media_types::table as my_table;
        use ::contactopensource::models::media_type::media_type::MediaType as MyModel;
        let x: MyModel = MyModel::fab();
        let json_value = x.as_serde_json_value();
        let insertable_struct: MyModel = serde_json::from_value::<MyModel>(json_value).unwrap();
        let query = diesel::insert_into(my_table).values(&insertable_struct);
        let exp = x.as_sql_insert();
        let act = diesel::debug_query::<Pg, _>(&query).to_string();
        assert_eq!(exp, act);
    }

    //// GeoPoint
    #[test]
    fn test_insert_geo_point_as_insertable_struct_test_sql() {
        use ::contactopensource::schema::geo_points::table as my_table;
        use ::contactopensource::models::geo_point::geo_point::GeoPoint as MyModel;
        let x: MyModel = MyModel::fab();
        let json_value = x.as_serde_json_value();
        let insertable_struct: MyModel = serde_json::from_value::<MyModel>(json_value).unwrap();
        let query = diesel::insert_into(my_table).values(&insertable_struct);
        let exp = x.as_sql_insert();
        let act = diesel::debug_query::<Pg, _>(&query).to_string();
        assert_eq!(exp, act);
    }

    //// GeoCode
    #[test]
    fn test_insert_geo_code_as_insertable_struct_test_sql() {
        use ::contactopensource::schema::geo_codes::table as my_table;
        use ::contactopensource::models::geo_code::geo_code::GeoCode as MyModel;
        let x: MyModel = MyModel::fab();
        let json_value = x.as_serde_json_value();
        let insertable_struct: MyModel = serde_json::from_value::<MyModel>(json_value).unwrap();
        let query = diesel::insert_into(my_table).values(&insertable_struct);
        let exp = x.as_sql_insert();
        let act = diesel::debug_query::<Pg, _>(&query).to_string();
        assert_eq!(exp, act);
    }

    //// Person Name
    #[test]
    fn test_insert_person_name_as_insertable_struct_test_sql() {
        use ::contactopensource::schema::person_names::table as my_table;
        use ::contactopensource::models::person_name::person_name::PersonName as MyModel;
        let x: MyModel = MyModel::fab();
        let json_value = x.as_serde_json_value();
        let insertable_struct: MyModel = serde_json::from_value::<MyModel>(json_value).unwrap();
        let query = diesel::insert_into(my_table).values(&insertable_struct);
        let exp = x.as_sql_insert();
        let act = diesel::debug_query::<Pg, _>(&query).to_string();
        assert_eq!(exp, act);
    }

    //// Person Pronoun
    #[test]
    fn test_insert_person_pronoun_as_insertable_struct_test_sql() {
        use ::contactopensource::schema::person_pronouns::table as my_table;
        use ::contactopensource::models::person_pronoun::person_pronoun::PersonPronoun as MyModel;
        let x: MyModel = MyModel::fab();
        let json_value = x.as_serde_json_value();
        let insertable_struct: MyModel = serde_json::from_value::<MyModel>(json_value).unwrap();
        let query = diesel::insert_into(my_table).values(&insertable_struct);
        let exp = x.as_sql_insert();
        let act = diesel::debug_query::<Pg, _>(&query).to_string();
        assert_eq!(exp, act);
    }

}