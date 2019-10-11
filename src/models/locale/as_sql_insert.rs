use crate::models::locale::locale::Locale;

impl crate::traits::as_sql_insert::AsSqlInsert for Locale {

    fn as_sql_insert(&self) -> String {
        return format!(
            "INSERT INTO \"locales\" (\
            \"id\", \
            \"tenant_id\", \
            \"typecast\", \
            \"state\", \
            \"updated_at_timestamp_utc\", \
            \"updated_at_clock_count\", \
            \"updated_by_text\", \
            \"text\", \
            \"language_code\", \
            \"country_code\", \
            \"script_code\", \
            \"region_code\", \
            \"variant_code\", \
            \"decimal_separator\", \
            \"grouping_separator\", \
            \"currency_code\", \
            \"currency_symbol\", \
            \"quotation_start_delimiter\", \
            \"quotation_stop_delimiter\"\
            ) \
            VALUES \
            ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19) \
            -- binds: [{:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}]",

            self.id.clone(),

            // Programming-related
            self.tenant_id.clone().unwrap(),
            self.typecast.clone().unwrap(),
            self.state.clone().unwrap(),

            // Update-related
            self.updated_at_timestamp_utc.clone().unwrap(),
            self.updated_at_clock_count.clone().unwrap(),
            self.updated_by_text.clone().unwrap(),

            // Code-related
            self.text.clone().unwrap(),
            self.language_code.clone().unwrap(),
            self.country_code.clone().unwrap(),
            self.script_code.clone().unwrap(),
            self.region_code.clone().unwrap(),
            self.variant_code.clone().unwrap(),

            // Representation-related
            self.decimal_separator.clone().unwrap(),
            self.grouping_separator.clone().unwrap(),
            self.currency_code.clone().unwrap(),
            self.currency_symbol.clone().unwrap(),
            self.quotation_start_delimiter.clone().unwrap(),
            self.quotation_stop_delimiter.clone().unwrap(),

        );
    }

}