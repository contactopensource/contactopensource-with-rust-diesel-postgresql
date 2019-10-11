GRANT ALL ON DATABASE contactopensource TO contactopensource_administrator;

GRANT CONNECT ON DATABASE contactopensource TO contactopensource_reader;
GRANT USAGE ON SCHEMA public TO contactopensource_reader;
GRANT SELECT ON ALL TABLES IN SCHEMA public TO contactopensource_reader;
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT SELECT ON TABLES TO contactopensource_reader;
