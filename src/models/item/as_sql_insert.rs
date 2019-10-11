use crate::models::item::item::Item;

impl crate::traits::as_sql_insert::AsSqlInsert for Item {

    fn as_sql_insert(&self) -> String {
        return format!(
            "INSERT INTO \"items\" (\
            \"id\", \
            \"tenant_id\", \
            \"typecast\", \
            \"state\", \
            \"updated_at_timestamp_utc\", \
            \"updated_at_clock_count\", \
            \"updated_by_text\", \
            \"uri\", \
            \"text\", \
            \"json\", \
            \"xml\", \
            \"number\"\
            ) \
            VALUES \
            ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12) \
            -- binds: [{:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}]",

            self.id.clone(),

            // Programming-related
            self.tenant_id.clone().unwrap(),
            self.typecast.clone().unwrap(),
            self.state.clone().unwrap(),

            // Update-related
            self.updated_at_timestamp_utc.clone().unwrap(),
            self.updated_at_clock_count.clone().unwrap(),
            self.updated_by_text.clone().unwrap(),

            // Meta-data-related
            self.uri.clone().unwrap(),

            // Content-related
            self.text.clone().unwrap(),
            self.json.clone().unwrap(),
            self.xml.clone().unwrap(),
            self.number.clone().unwrap(),

        );
    }

}