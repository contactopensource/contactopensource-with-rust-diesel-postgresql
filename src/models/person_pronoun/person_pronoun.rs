use crate::types;
use crate::schema::person_pronouns;

#[derive(Debug, Identifiable, Deserialize, Queryable, PartialEq, Insertable, AsChangeset)]
#[table_name = "person_pronouns"]
pub struct PersonPronoun {
    pub id: types::id::Id,

    // Programming-related
    pub tenant_id: Option<types::id::Id>,
    pub typecast: Option<types::typecast::Typecast>,
    pub state: Option<types::state::State>,

    // Update-related
    pub updated_at_timestamp_utc: Option<types::timestamp::Timestamp>,
    pub updated_at_clock_count: Option<types::count::Count>,
    pub updated_by_text: Option<types::text::Text>,

    // Pronoun-related
    pub subject_pronoun: Option<types::subject_pronoun::SubjectPronoun>,
    pub object_pronoun: Option<types::object_pronoun::ObjectPronoun>,
    pub dependent_possessive_pronoun: Option<types::dependent_possessive_pronoun::DependentPossessivePronoun>,
    pub independent_possessive_pronoun: Option<types::independent_possessive_pronoun::IndependentPossessivePronoun>,
    pub reflexive_pronoun: Option<types::reflexive_pronoun::ReflexivePronoun>,
    pub intensive_pronoun: Option<types::intensive_pronoun::IntensivePronoun>,
    pub disjunctive_pronoun: Option<types::disjunctive_pronoun::DisjunctivePronoun>,

}
