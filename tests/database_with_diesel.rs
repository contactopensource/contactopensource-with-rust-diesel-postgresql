//! Test insert with Diesel

#[cfg(test)]
mod test_database_with_diesel {

    extern crate contactopensource;

    use ::diesel::pg::Pg;

    use ::contactopensource::traits::as_serde_json_value::AsSerdeJsonValue;
    use ::contactopensource::traits::as_sql_insert::AsSqlInsert;

    use ::contactopensource::models::item::item::*;
    use ::contactopensource::models::arc::arc::*;
    use ::contactopensource::models::org::org::*;
    use ::contactopensource::models::person::person::*;
    use ::contactopensource::models::place::place::*;
    use ::contactopensource::models::thing::thing::*;
    use ::contactopensource::models::event::event::*;
    use ::contactopensource::models::link_contact::link_contact::*;
    use ::contactopensource::models::email_contact::email_contact::*;
    use ::contactopensource::models::telephone_contact::telephone_contact::*;
    use ::contactopensource::models::passport_contact::passport_contact::*;
    use ::contactopensource::models::postal_contact::postal_contact::*;
    use ::contactopensource::models::contact::contact::*;
    use ::contactopensource::models::tag::tag::*;
    use ::contactopensource::models::code::code::*;
    use ::contactopensource::models::locale::locale::*;
    use ::contactopensource::models::media_type::media_type::*;
    use ::contactopensource::models::geo_point::geo_point::*;
    use ::contactopensource::models::geo_code::geo_code::*;
    use ::contactopensource::models::person_name::person_name::*;
    use ::contactopensource::models::person_pronoun::person_pronoun::*;

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
        use ::contactopensource::schema::items::dsl::*;
        let x = ::contactopensource::models::item::fab::fab();
        let json_value = x.as_serde_json_value();
        let insertable_struct: Item = serde_json::from_value::<Item>(json_value).unwrap();
        let query = diesel::insert_into(items).values(&insertable_struct);
        let exp = x.as_sql_insert();
        let act = diesel::debug_query::<Pg, _>(&query).to_string();
        assert_eq!(exp, act);
    }

    //// Arc
    #[test]
    fn test_insert_arc_as_insertable_struct_test_sql() {
        use ::contactopensource::schema::arcs::dsl::*;
        let x = ::contactopensource::models::arc::fab::fab();
        let json_value = x.as_serde_json_value();
        let insertable_struct: Arc = serde_json::from_value::<Arc>(json_value).unwrap();
        let query = diesel::insert_into(arcs).values(&insertable_struct);
        let exp = x.as_sql_insert();
        let act = diesel::debug_query::<Pg, _>(&query).to_string();
        assert_eq!(exp, act);
    }

    //// Org
    #[test]
    fn test_insert_org_as_insertable_struct_test_sql() {
        use ::contactopensource::schema::orgs::dsl::*;
        let x = ::contactopensource::models::org::fab::fab();
        let json_value = x.as_serde_json_value();
        let insertable_struct: Org = serde_json::from_value::<Org>(json_value).unwrap();
        let query = diesel::insert_into(orgs).values(&insertable_struct);
        let exp = x.as_sql_insert();
        let act = diesel::debug_query::<Pg, _>(&query).to_string();
        assert_eq!(exp, act);
    }

    //// Person
    #[test]
    fn test_insert_person_as_insertable_struct_test_sql() {
        use ::contactopensource::schema::persons::dsl::*;
        let x = ::contactopensource::models::person::fab::fab();
        let json_value = x.as_serde_json_value();
        let insertable_struct: Person = serde_json::from_value::<Person>(json_value).unwrap();
        let query = diesel::insert_into(persons).values(&insertable_struct);
        let exp = x.as_sql_insert();
        let act = diesel::debug_query::<Pg, _>(&query).to_string();
        assert_eq!(exp, act);
    }

    //// Place
    #[test]
    fn test_insert_place_as_insertable_struct_test_sql() {
        use ::contactopensource::schema::places::dsl::*;
        let x = ::contactopensource::models::place::fab::fab();
        let json_value = x.as_serde_json_value();
        let insertable_struct: Place = serde_json::from_value::<Place>(json_value).unwrap();
        let query = diesel::insert_into(places).values(&insertable_struct);
        let exp = x.as_sql_insert();
        let act = diesel::debug_query::<Pg, _>(&query).to_string();
        assert_eq!(exp, act);
    }

    //// Thing
    #[test]
    fn test_insert_thing_as_insertable_struct_test_sql() {
        use ::contactopensource::schema::things::dsl::*;
        let x = ::contactopensource::models::thing::fab::fab();
        let json_value = x.as_serde_json_value();
        let insertable_struct: Thing = serde_json::from_value::<Thing>(json_value).unwrap();
        let query = diesel::insert_into(things).values(&insertable_struct);
        let exp = x.as_sql_insert();
        let act = diesel::debug_query::<Pg, _>(&query).to_string();
        assert_eq!(exp, act);
    }

    //// Event
    #[test]
    fn test_insert_event_as_insertable_struct_test_sql() {
        use ::contactopensource::schema::events::dsl::*;
        let x = ::contactopensource::models::event::fab::fab();
        let json_value = x.as_serde_json_value();
        let insertable_struct: Event = serde_json::from_value::<Event>(json_value).unwrap();
        let query = diesel::insert_into(events).values(insertable_struct);
        let exp = x.as_sql_insert();
        let act = diesel::debug_query::<Pg, _>(&query).to_string();
        assert_eq!(exp, act);
    }

    //// LinkContact
    #[test]
    fn test_insert_link_contact_as_insertable_struct_test_sql() {
        use ::contactopensource::schema::link_contacts::dsl::*;
        let x = ::contactopensource::models::link_contact::fab::fab();
        let json_value = x.as_serde_json_value();
        let insertable_struct: LinkContact = serde_json::from_value::<LinkContact>(json_value).unwrap();
        let query = diesel::insert_into(link_contacts).values(insertable_struct);
        let exp = x.as_sql_insert();
        let act = diesel::debug_query::<Pg, _>(&query).to_string();
        assert_eq!(exp, act);
    }

    //// EmailContact
    #[test]
    fn test_insert_email_contact_as_insertable_struct_test_sql() {
        use ::contactopensource::schema::email_contacts::dsl::*;
        let x = ::contactopensource::models::email_contact::fab::fab();
        let json_value = x.as_serde_json_value();
        let insertable_struct: EmailContact = serde_json::from_value::<EmailContact>(json_value).unwrap();
        let query = diesel::insert_into(email_contacts).values(insertable_struct);
        let exp = x.as_sql_insert();
        let act = diesel::debug_query::<Pg, _>(&query).to_string();
        assert_eq!(exp, act);
    }

    //// TelephoneContact
    #[test]
    fn test_insert_telephone_contact_as_insertable_struct_test_sql() {
        use ::contactopensource::schema::telephone_contacts::dsl::*;
        let x = ::contactopensource::models::telephone_contact::fab::fab();
        let json_value = x.as_serde_json_value();
        let insertable_struct: TelephoneContact = serde_json::from_value::<TelephoneContact>(json_value).unwrap();
        let query = diesel::insert_into(telephone_contacts).values(insertable_struct);
        let exp = x.as_sql_insert();
        let act = diesel::debug_query::<Pg, _>(&query).to_string();
        assert_eq!(exp, act);
    }

    //// PostalContact
    #[test]
    fn test_insert_postal_contact_as_insertable_struct_test_sql() {
        use ::contactopensource::schema::postal_contacts::dsl::*;
        let x = ::contactopensource::models::postal_contact::fab::fab();
        let json_value = x.as_serde_json_value();
        let insertable_struct: PostalContact = serde_json::from_value::<PostalContact>(json_value).unwrap();
        let query = diesel::insert_into(postal_contacts).values(insertable_struct);
        let exp = x.as_sql_insert();
        let act = diesel::debug_query::<Pg, _>(&query).to_string();
        assert_eq!(exp, act);
    }

    //// PassportContact
    #[test]
    fn test_insert_passport_contact_as_insertable_struct_test_sql() {
        use ::contactopensource::schema::passport_contacts::dsl::*;
        let x = ::contactopensource::models::passport_contact::fab::fab();
        let json_value = x.as_serde_json_value();
        let insertable_struct: PassportContact = serde_json::from_value::<PassportContact>(json_value).unwrap();
        let query = diesel::insert_into(passport_contacts).values(insertable_struct);
        let exp = x.as_sql_insert();
        let act = diesel::debug_query::<Pg, _>(&query).to_string();
        assert_eq!(exp, act);
    }

    //// Contact
    #[test]
    fn test_insert_contact_as_insertable_struct_test_sql() {
        use ::contactopensource::schema::contacts::dsl::*;
        let x = ::contactopensource::models::contact::fab::fab();
        let json_value = x.as_serde_json_value();
        let insertable_struct: Contact = serde_json::from_value::<Contact>(json_value).unwrap();
        let query = diesel::insert_into(contacts).values(&insertable_struct);
        let exp = x.as_sql_insert();
        let act = diesel::debug_query::<Pg, _>(&query).to_string();
        assert_eq!(exp, act);
    }

    //// Tag
    #[test]
    fn test_insert_tag_as_insertable_struct_test_sql() {
        use ::contactopensource::schema::tags::dsl::*;
        let x = ::contactopensource::models::tag::fab::fab();
        let json_value = x.as_serde_json_value();
        let insertable_struct: Tag = serde_json::from_value::<Tag>(json_value).unwrap();
        let query = diesel::insert_into(tags).values(&insertable_struct);
        let exp = x.as_sql_insert();
        let act = diesel::debug_query::<Pg, _>(&query).to_string();
        assert_eq!(exp, act);
    }

    //// Code
    #[test]
    fn test_insert_code_as_insertable_struct_test_sql() {
        use ::contactopensource::schema::codes::dsl::*;
        let x = ::contactopensource::models::code::fab::fab();
        let json_value = x.as_serde_json_value();
        let insertable_struct: Code = serde_json::from_value::<Code>(json_value).unwrap();
        let query = diesel::insert_into(codes).values(&insertable_struct);
        let exp = x.as_sql_insert();
        let act = diesel::debug_query::<Pg, _>(&query).to_string();
        assert_eq!(exp, act);
    }

    //// Locale
    #[test]
    fn test_insert_locale_as_insertable_struct_test_sql() {
        use ::contactopensource::schema::locales::dsl::*;
        let x = ::contactopensource::models::locale::fab::fab();
        let json_value = x.as_serde_json_value();
        let insertable_struct: Locale = serde_json::from_value::<Locale>(json_value).unwrap();
        let query = diesel::insert_into(locales).values(&insertable_struct);
        let exp = x.as_sql_insert();
        let act = diesel::debug_query::<Pg, _>(&query).to_string();
        assert_eq!(exp, act);
    }

    //// MediaType
    #[test]
    fn test_insert_media_type_as_insertable_struct_test_sql() {
        use ::contactopensource::schema::media_types::dsl::*;
        let x = ::contactopensource::models::media_type::fab::fab();
        let json_value = x.as_serde_json_value();
        let insertable_struct: MediaType = serde_json::from_value::<MediaType>(json_value).unwrap();
        let query = diesel::insert_into(media_types).values(&insertable_struct);
        let exp = x.as_sql_insert();
        let act = diesel::debug_query::<Pg, _>(&query).to_string();
        assert_eq!(exp, act);
    }

    //// GeoPoint
    #[test]
    fn test_insert_geo_point_as_insertable_struct_test_sql() {
        use ::contactopensource::schema::geo_points::dsl::*;
        let x = ::contactopensource::models::geo_point::fab::fab();
        let json_value = x.as_serde_json_value();
        let insertable_struct: GeoPoint = serde_json::from_value::<GeoPoint>(json_value).unwrap();
        let query = diesel::insert_into(geo_points).values(insertable_struct);
        let exp = x.as_sql_insert();
        let act = diesel::debug_query::<Pg, _>(&query).to_string();
        assert_eq!(exp, act);
    }

    //// GeoCode
    #[test]
    fn test_insert_geo_code_as_insertable_struct_test_sql() {
        use ::contactopensource::schema::geo_codes::dsl::*;
        let x = ::contactopensource::models::geo_code::fab::fab();
        let json_value = x.as_serde_json_value();
        let insertable_struct: GeoCode = serde_json::from_value::<GeoCode>(json_value).unwrap();
        let query = diesel::insert_into(geo_codes).values(insertable_struct);
        let exp = x.as_sql_insert();
        let act = diesel::debug_query::<Pg, _>(&query).to_string();
        assert_eq!(exp, act);
    }

    //// Person Name
    #[test]
    fn test_insert_person_name_as_insertable_struct_test_sql() {
        use ::contactopensource::schema::person_names::dsl::*;
        let x = ::contactopensource::models::person_name::fab::fab();
        let json_value = x.as_serde_json_value();
        let insertable_struct: PersonName = serde_json::from_value::<PersonName>(json_value).unwrap();
        let query = diesel::insert_into(person_names).values(&insertable_struct);
        let exp = x.as_sql_insert();
        let act = diesel::debug_query::<Pg, _>(&query).to_string();
        assert_eq!(exp, act);
    }

    //// Person Pronoun
    #[test]
    fn test_insert_person_pronoun_as_insertable_struct_test_sql() {
        use ::contactopensource::schema::person_pronouns::dsl::*;
        let x = ::contactopensource::models::person_pronoun::fab::fab();
        let json_value = x.as_serde_json_value();
        let insertable_struct: PersonPronoun = serde_json::from_value::<PersonPronoun>(json_value).unwrap();
        let query = diesel::insert_into(person_pronouns).values(&insertable_struct);
        let exp = x.as_sql_insert();
        let act = diesel::debug_query::<Pg, _>(&query).to_string();
        assert_eq!(exp, act);
    }

}