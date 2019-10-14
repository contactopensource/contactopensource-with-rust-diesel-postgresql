use crate::types;
use crate::models::person_pronoun::person_pronoun::PersonPronoun as T;

impl crate::traits::fab_able::FabAble for T {
    fn fab() -> T {
        Self {
            id: types::id::fab(),

            // Programming-related
            tenant_id: Some(types::id::fab()),
            typecast: Some(types::typecast::fab()),
            state: Some(types::state::fab()),

            // Update-related
            updated_at_timestamp_utc: Some(types::timestamp::fab()),
            updated_at_clock_count: Some(types::count::fab()),
            updated_by_text: Some(types::text::fab()),

            // Pronoun-related
            subject_pronoun: Some(types::subject_pronoun::fab()),
            object_pronoun: Some(types::object_pronoun::fab()),
            dependent_possessive_pronoun: Some(types::dependent_possessive_pronoun::fab()),
            independent_possessive_pronoun: Some(types::independent_possessive_pronoun::fab()),
            reflexive_pronoun: Some(types::reflexive_pronoun::fab()),
            intensive_pronoun: Some(types::intensive_pronoun::fab()),
            disjunctive_pronoun: Some(types::disjunctive_pronoun::fab()),

        }
    }
}
