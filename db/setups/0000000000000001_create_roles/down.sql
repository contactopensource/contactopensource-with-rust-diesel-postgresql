REVOKE ALL PRIVILEGES ON ALL TABLES IN SCHEMA public FROM contactopensource_reader;
REVOKE ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public FROM contactopensource_reader;
REVOKE ALL PRIVILEGES ON ALL FUNCTIONS IN SCHEMA public FROM contactopensource_reader;
REVOKE USAGE ON SCHEMA public FROM contactopensource_reader;
REVOKE ALL PRIVILEGES ON DATABASE contactopensource FROM contactopensource_reader;
DROP OWNED BY contactopensource_reader;
REASSIGN OWNED BY contactopensource_reader TO postgres;
DROP ROLE contactopensource_reader;

REVOKE ALL PRIVILEGES ON ALL TABLES IN SCHEMA public FROM contactopensource_reader_administrator;
REVOKE ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public FROM contactopensource_reader_administrator;
REVOKE ALL PRIVILEGES ON ALL FUNCTIONS IN SCHEMA public FROM contactopensource_administrator;
REVOKE USAGE ON SCHEMA public FROM contactopensource_administrator;
REVOKE ALL PRIVILEGES ON DATABASE contactopensource FROM contactopensource_administrator;
DROP OWNED BY contactopensource_administrator;
REASSIGN OWNED BY contactopensource_administrator; TO postgres;
DROP ROLE contactopensource_administrator;
