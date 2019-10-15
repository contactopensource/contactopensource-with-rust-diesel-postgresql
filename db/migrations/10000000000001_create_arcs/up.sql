-- This 'arcs' table is a kind of linking table with special capabilties.
-- An arc row can associate up to three concepts: subject, predicate, object.
--
-- This kind of association has similarities to a data "semantic triple",
-- and the metadata data model Resource Description Framework (RDF).
--
--   * See https://en.wikipedia.org/wiki/Semantic_triple
--
--   * See https://en.wikipedia.org/wiki/Resource_Description_Framework
--
-- Each concept can be a either a URI or a database table name and row id.
-- This enables links similar to RDF triples or polymorphic joins.
--
-- For example, link two URIs anywhere on the internet, such as two people:
--
--   * subject_uri: http://example.com/alice-adams.html
--
--   * object_uri: http://example.com/bob-brown.html
-- 
-- For example, link from a person's information in this database schema
-- to an organization's information in this database schema:
--
--    * subject_table: persons
--
--    * subject_id: 34b75621921fdc7ac83459c5c4b7dba6
--
--    * object_table: orgs
--
--    * object_id: 9588686d2a1b4cda40cad5269c87a627
--
-- For example, link a person in this database schema to an external URI:
--
--    * subject_table: persons
--
--    * subject_id: 34b75621921fdc7ac83459c5c4b7dba6
--
--    * object_uri: http://example.com/bob-brown.html
--
-- Each arc row must have a subject and object. 
--
-- Each arc row may have a predicate, which is a way to describe the type
-- of relationship. Common examples we use are "like", "follow", "favorite".
--
-- This table has a goal to be fully workable with RDF concepts.
--
-- If possible, this table will be improved to use a better way of 
-- linking to internal data. For example, we would prefer to phase out
-- the individual fields for table name and row id, and instead simply
-- use the URI fields along with a URI that points to internal data.
-- Currently, many tools and frameworks expect separate fields for
-- the table name and row id, such as to do poly-to-poly joins.

CREATE TABLE arcs (
  id uuid not null primary key,

  -- Programming-related
  tenant_id uuid, -- for optional multi-tenant installation
  typecast text, -- for optional single table inheritance
  state text, -- for optional state machine transition

  -- Update-related
  updated_at_timestamp_utc timestamp, -- example: 2020-01-01T00:00:00 always UTC
  updated_at_clock_count bigint, -- example: 123456789 as suitable for a vector clock
  updated_by_text text, -- example: explanation of who updated the contact, why, how, etc.

  -- Subject
  subject_uri text, -- example: http://example.com/alpha.html
  subject_database text, -- database table name; example: 'contactopensource'
  subject_schema text, -- database table name; example: 'public'
  subject_table text, -- database table name; example: 'persons'
  subject_id uuid, -- database row id; example: 34b75621921fdc7ac83459c5c4b7dba6

  -- Predicate
  predicate_uri text, -- example: http://example.com/bravo.html
  predicate_database text, -- database table name; example: 'contactopensource'
  predicate_schema text, -- database table name; example: 'public'
  predicate_table text, -- database table name; example: 'likes'
  predicate_id uuid, -- database row id; example: 124cf87662601612ae47379c91876e1e

  -- Object
  object_uri text, -- example: http://example.com/charlie.html
  object_database text, -- database table name; example: 'contactopensource'
  object_schema text, -- database table name; example: 'public'
  object_table text, -- database table name; example: 'orgs'
  object_id uuid, -- database row id; example: 9588686d2a1b4cda40cad5269c87a627

  -- Lifecycle
  start_at_timestamp_utc timestamp,
  stop_at_timestamp_utc timestamp,

  -- Modifiers
  count bigint, -- count, such as an instance index; example: 10 means count 10
  weight numeric(10,9), -- weight, -1 to 1 inclusive; example: 0.1 means weight 10%
  probability numeric(10,9) -- probability, 0 to 1 inclusive; example: 0.1 means probability 10%

);

-- Programming-related
CREATE INDEX ix_arcs_tenant_id on arcs(tenant_id);
CREATE INDEX ix_arcs_typecast on arcs(typecast);
CREATE INDEX ix_arcs_state on arcs(state);

-- Update-related
CREATE INDEX ix_arcs_updated_at_timestamp_utc on arcs(updated_at_timestamp_utc);
CREATE INDEX ix_arcs_updated_at_clock_count on arcs(updated_at_clock_count);
CREATE INDEX ix_arcs_updated_by_text on arcs(updated_by_text);

-- Subject
CREATE INDEX ix_arcs_subject_uri on arcs(subject_uri);
CREATE INDEX ix_arcs_subject_database on arcs(subject_database);
CREATE INDEX ix_arcs_subject_schema on arcs(subject_schema);
CREATE INDEX ix_arcs_subject_table on arcs(subject_table);
CREATE INDEX ix_arcs_subject_id on arcs(subject_id);

-- Predicate
CREATE INDEX ix_arcs_predicate_uri on arcs(predicate_uri);
CREATE INDEX ix_arcs_predicate_database on arcs(predicate_database);
CREATE INDEX ix_arcs_predicate_schema on arcs(predicate_schema);
CREATE INDEX ix_arcs_predicate_table on arcs(predicate_table);
CREATE INDEX ix_arcs_predicate_id on arcs(predicate_id);

-- Object
CREATE INDEX ix_arcs_object_uri on arcs(object_uri);
CREATE INDEX ix_arcs_object_database on arcs(object_database);
CREATE INDEX ix_arcs_object_schema on arcs(object_schema);
CREATE INDEX ix_arcs_object_table on arcs(object_table);
CREATE INDEX ix_arcs_object_id on arcs(object_id);

-- Lifecycle
CREATE INDEX ix_arcs_start_at_timestamp_utc on arcs(start_at_timestamp_utc);
CREATE INDEX ix_arcs_stop_at_timestamp_utc on arcs(stop_at_timestamp_utc);

-- Modifiers
CREATE INDEX ix_arcs_count on arcs(count);
CREATE INDEX ix_arcs_weight on arcs(weight);
CREATE INDEX ix_arcs_probability on arcs(probability);
