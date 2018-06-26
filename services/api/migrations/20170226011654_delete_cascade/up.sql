ALTER TABLE info_uri DROP CONSTRAINT info_uri_series_id_fkey;
ALTER TABLE info_uri ADD CONSTRAINT info_uri_series_id_fkey FOREIGN KEY (series_id) REFERENCES series(id) ON DELETE CASCADE;
