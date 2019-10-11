table! {
    arcs (id) {
        id -> Uuid,
        tenant_id -> Nullable<Uuid>,
        typecast -> Nullable<Text>,
        state -> Nullable<Text>,
        updated_at_timestamp_utc -> Nullable<Timestamp>,
        updated_at_clock_count -> Nullable<Int8>,
        updated_by_text -> Nullable<Text>,
        subject_uri -> Nullable<Text>,
        subject_database -> Nullable<Text>,
        subject_schema -> Nullable<Text>,
        subject_table -> Nullable<Text>,
        subject_id -> Nullable<Uuid>,
        predicate_uri -> Nullable<Text>,
        predicate_database -> Nullable<Text>,
        predicate_schema -> Nullable<Text>,
        predicate_table -> Nullable<Text>,
        predicate_id -> Nullable<Uuid>,
        object_uri -> Nullable<Text>,
        object_database -> Nullable<Text>,
        object_schema -> Nullable<Text>,
        object_table -> Nullable<Text>,
        object_id -> Nullable<Uuid>,
        start_at_timestamp_utc -> Nullable<Timestamp>,
        stop_at_timestamp_utc -> Nullable<Timestamp>,
        count -> Nullable<Int8>,
        weight -> Nullable<Numeric>,
        probability -> Nullable<Numeric>,
    }
}

table! {
    codes (id) {
        id -> Uuid,
        tenant_id -> Nullable<Uuid>,
        typecast -> Nullable<Text>,
        state -> Nullable<Text>,
        updated_at_timestamp_utc -> Nullable<Timestamp>,
        updated_at_clock_count -> Nullable<Int8>,
        updated_by_text -> Nullable<Text>,
        set_id -> Nullable<Uuid>,
        text -> Nullable<Text>,
        name -> Nullable<Text>,
    }
}

table! {
    contacts (id) {
        id -> Uuid,
        tenant_id -> Nullable<Uuid>,
        typecast -> Nullable<Text>,
        state -> Nullable<Text>,
        updated_at_timestamp_utc -> Nullable<Timestamp>,
        updated_at_clock_count -> Nullable<Int8>,
        updated_by_text -> Nullable<Text>,
        name -> Nullable<Text>,
        emoji -> Nullable<Text>,
        image_uri -> Nullable<Text>,
        color_hex -> Nullable<Text>,
        css_class -> Nullable<Text>,
        star_count -> Nullable<Int2>,
    }
}

table! {
    email_contacts (id) {
        id -> Uuid,
        tenant_id -> Nullable<Uuid>,
        typecast -> Nullable<Text>,
        state -> Nullable<Text>,
        updated_at_timestamp_utc -> Nullable<Timestamp>,
        updated_at_clock_count -> Nullable<Int8>,
        updated_by_text -> Nullable<Text>,
        address -> Nullable<Text>,
        display_name -> Nullable<Text>,
        addr_spec -> Nullable<Text>,
    }
}

table! {
    events (id) {
        id -> Uuid,
        tenant_id -> Nullable<Uuid>,
        typecast -> Nullable<Text>,
        state -> Nullable<Text>,
        updated_at_timestamp_utc -> Nullable<Timestamp>,
        updated_at_clock_count -> Nullable<Int8>,
        updated_by_text -> Nullable<Text>,
        name -> Nullable<Text>,
        start_timestamp_utc -> Nullable<Timestamp>,
        stop_timestamp_utc -> Nullable<Timestamp>,
        duration_as_seconds -> Nullable<Numeric>,
    }
}

table! {
    geo_codes (id) {
        id -> Uuid,
        tenant_id -> Nullable<Uuid>,
        typecast -> Nullable<Text>,
        state -> Nullable<Text>,
        updated_at_timestamp_utc -> Nullable<Timestamp>,
        updated_at_clock_count -> Nullable<Int8>,
        updated_by_text -> Nullable<Text>,
        coder_id -> Nullable<Uuid>,
        text -> Nullable<Text>,
        latitude -> Nullable<Numeric>,
        longitude -> Nullable<Numeric>,
        altitude -> Nullable<Numeric>,
        elevation -> Nullable<Numeric>,
    }
}

table! {
    geo_points (id) {
        id -> Uuid,
        tenant_id -> Nullable<Uuid>,
        typecast -> Nullable<Text>,
        state -> Nullable<Text>,
        updated_at_timestamp_utc -> Nullable<Timestamp>,
        updated_at_clock_count -> Nullable<Int8>,
        updated_by_text -> Nullable<Text>,
        latitude -> Nullable<Numeric>,
        longitude -> Nullable<Numeric>,
        altitude -> Nullable<Numeric>,
        elevation -> Nullable<Numeric>,
    }
}

table! {
    items (id) {
        id -> Uuid,
        tenant_id -> Nullable<Uuid>,
        typecast -> Nullable<Text>,
        state -> Nullable<Text>,
        updated_at_timestamp_utc -> Nullable<Timestamp>,
        updated_at_clock_count -> Nullable<Int8>,
        updated_by_text -> Nullable<Text>,
        uri -> Nullable<Text>,
        text -> Nullable<Text>,
        json -> Nullable<Jsonb>,
        xml -> Nullable<Text>,
        number -> Nullable<Numeric>,
    }
}

table! {
    link_contacts (id) {
        id -> Uuid,
        tenant_id -> Nullable<Uuid>,
        typecast -> Nullable<Text>,
        state -> Nullable<Text>,
        updated_at_timestamp_utc -> Nullable<Timestamp>,
        updated_at_clock_count -> Nullable<Int8>,
        updated_by_text -> Nullable<Text>,
        label -> Nullable<Text>,
        uri -> Nullable<Text>,
    }
}

table! {
    locales (id) {
        id -> Uuid,
        tenant_id -> Nullable<Uuid>,
        typecast -> Nullable<Text>,
        state -> Nullable<Text>,
        updated_at_timestamp_utc -> Nullable<Timestamp>,
        updated_at_clock_count -> Nullable<Int8>,
        updated_by_text -> Nullable<Text>,
        text -> Nullable<Text>,
        language_code -> Nullable<Text>,
        country_code -> Nullable<Text>,
        script_code -> Nullable<Text>,
        region_code -> Nullable<Text>,
        variant_code -> Nullable<Text>,
        decimal_separator -> Nullable<Text>,
        grouping_separator -> Nullable<Text>,
        currency_code -> Nullable<Text>,
        currency_symbol -> Nullable<Text>,
        quotation_start_delimiter -> Nullable<Text>,
        quotation_stop_delimiter -> Nullable<Text>,
    }
}

table! {
    media_types (id) {
        id -> Uuid,
        tenant_id -> Nullable<Uuid>,
        typecast -> Nullable<Text>,
        state -> Nullable<Text>,
        updated_at_timestamp_utc -> Nullable<Timestamp>,
        updated_at_clock_count -> Nullable<Int8>,
        updated_by_text -> Nullable<Text>,
        text -> Nullable<Text>,
        supertype -> Nullable<Text>,
        subtype -> Nullable<Text>,
        tree -> Nullable<Text>,
        suffix -> Nullable<Text>,
        parameters -> Nullable<Array<Text>>,
    }
}

table! {
    orgs (id) {
        id -> Uuid,
        tenant_id -> Nullable<Uuid>,
        typecast -> Nullable<Text>,
        state -> Nullable<Text>,
        updated_at_timestamp_utc -> Nullable<Timestamp>,
        updated_at_clock_count -> Nullable<Int8>,
        updated_by_text -> Nullable<Text>,
        name -> Nullable<Text>,
        emoji -> Nullable<Text>,
        image_uri -> Nullable<Text>,
        color_hex -> Nullable<Text>,
        css_class -> Nullable<Text>,
        star_count -> Nullable<Int2>,
        start_date -> Nullable<Date>,
        stop_date -> Nullable<Date>,
    }
}

table! {
    passport_contacts (id) {
        id -> Uuid,
        tenant_id -> Nullable<Uuid>,
        typecast -> Nullable<Text>,
        state -> Nullable<Text>,
        updated_at_timestamp_utc -> Nullable<Timestamp>,
        updated_at_clock_count -> Nullable<Int8>,
        updated_by_text -> Nullable<Text>,
        country_text -> Nullable<Text>,
        number_text -> Nullable<Text>,
        valid_start_date -> Nullable<Date>,
        valid_stop_date -> Nullable<Date>,
    }
}

table! {
    person_names (id) {
        id -> Uuid,
        tenant_id -> Nullable<Uuid>,
        typecast -> Nullable<Text>,
        state -> Nullable<Text>,
        updated_at_timestamp_utc -> Nullable<Timestamp>,
        updated_at_clock_count -> Nullable<Int8>,
        updated_by_text -> Nullable<Text>,
        given_name -> Nullable<Text>,
        given_name_phonetic -> Nullable<Text>,
        middle_name -> Nullable<Text>,
        middle_name_phonetic -> Nullable<Text>,
        family_name -> Nullable<Text>,
        family_name_phonetic -> Nullable<Text>,
        legal_name -> Nullable<Text>,
        legal_name_phonetic -> Nullable<Text>,
        prefix_name -> Nullable<Text>,
        prefix_name_phonetic -> Nullable<Text>,
        suffix_name -> Nullable<Text>,
        suffix_name_phonetic -> Nullable<Text>,
        salutation_name -> Nullable<Text>,
        salutation_name_phonetic -> Nullable<Text>,
        addressee_name -> Nullable<Text>,
        addressee_name_phonetic -> Nullable<Text>,
        nickname -> Nullable<Text>,
        nickname_phonetic -> Nullable<Text>,
    }
}

table! {
    person_pronouns (id) {
        id -> Uuid,
        tenant_id -> Nullable<Uuid>,
        typecast -> Nullable<Text>,
        state -> Nullable<Text>,
        updated_at_timestamp_utc -> Nullable<Timestamp>,
        updated_at_clock_count -> Nullable<Int8>,
        updated_by_text -> Nullable<Text>,
        subject_pronoun -> Nullable<Text>,
        object_pronoun -> Nullable<Text>,
        dependent_possessive_pronoun -> Nullable<Text>,
        independent_possessive_pronoun -> Nullable<Text>,
        reflexive_pronoun -> Nullable<Text>,
        intensive_pronoun -> Nullable<Text>,
        disjunctive_pronoun -> Nullable<Text>,
    }
}

table! {
    persons (id) {
        id -> Uuid,
        tenant_id -> Nullable<Uuid>,
        typecast -> Nullable<Text>,
        state -> Nullable<Text>,
        updated_at_timestamp_utc -> Nullable<Timestamp>,
        updated_at_clock_count -> Nullable<Int8>,
        updated_by_text -> Nullable<Text>,
        birth_date -> Nullable<Date>,
        death_date -> Nullable<Date>,
        mass_as_grams -> Nullable<Numeric>,
        height_as_meters -> Nullable<Numeric>,
        org_name -> Nullable<Text>,
        org_team -> Nullable<Text>,
        org_role -> Nullable<Text>,
    }
}

table! {
    places (id) {
        id -> Uuid,
        tenant_id -> Nullable<Uuid>,
        typecast -> Nullable<Text>,
        state -> Nullable<Text>,
        updated_at_timestamp_utc -> Nullable<Timestamp>,
        updated_at_clock_count -> Nullable<Int8>,
        updated_by_text -> Nullable<Text>,
        name -> Nullable<Text>,
    }
}

table! {
    postal_contacts (id) {
        id -> Uuid,
        tenant_id -> Nullable<Uuid>,
        typecast -> Nullable<Text>,
        state -> Nullable<Text>,
        updated_at_timestamp_utc -> Nullable<Timestamp>,
        updated_at_clock_count -> Nullable<Int8>,
        updated_by_text -> Nullable<Text>,
        country_text -> Nullable<Text>,
        region_text -> Nullable<Text>,
        locality_text -> Nullable<Text>,
        neighborhood_text -> Nullable<Text>,
        postal_code_text -> Nullable<Text>,
        street_address_text -> Nullable<Text>,
        premise_address_text -> Nullable<Text>,
        global_location_number_text -> Nullable<Text>,
    }
}

table! {
    tags (id) {
        id -> Uuid,
        tenant_id -> Nullable<Uuid>,
        typecast -> Nullable<Text>,
        state -> Nullable<Text>,
        updated_at_timestamp_utc -> Nullable<Timestamp>,
        updated_at_clock_count -> Nullable<Int8>,
        updated_by_text -> Nullable<Text>,
        text -> Nullable<Text>,
    }
}

table! {
    telephone_contacts (id) {
        id -> Uuid,
        tenant_id -> Nullable<Uuid>,
        typecast -> Nullable<Text>,
        state -> Nullable<Text>,
        updated_at_timestamp_utc -> Nullable<Timestamp>,
        updated_at_clock_count -> Nullable<Int8>,
        updated_by_text -> Nullable<Text>,
        label -> Nullable<Text>,
        number_text -> Nullable<Text>,
        e164_text -> Nullable<Varchar>,
        e164_country_code -> Nullable<Varchar>,
        e164_national_destination_code -> Nullable<Varchar>,
        e164_group_identification_code -> Nullable<Varchar>,
        e164_trial_identification_code -> Nullable<Varchar>,
        e164_subscriber_number -> Nullable<Varchar>,
    }
}

table! {
    things (id) {
        id -> Uuid,
        tenant_id -> Nullable<Uuid>,
        typecast -> Nullable<Text>,
        state -> Nullable<Text>,
        updated_at_timestamp_utc -> Nullable<Timestamp>,
        updated_at_clock_count -> Nullable<Int8>,
        updated_by_text -> Nullable<Text>,
        name -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    arcs,
    codes,
    contacts,
    email_contacts,
    events,
    geo_codes,
    geo_points,
    items,
    link_contacts,
    locales,
    media_types,
    orgs,
    passport_contacts,
    person_names,
    person_pronouns,
    persons,
    places,
    postal_contacts,
    tags,
    telephone_contacts,
    things,
);
