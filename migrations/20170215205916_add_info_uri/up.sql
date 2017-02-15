CREATE TABLE info_uri (
    id SERIAL PRIMARY KEY,
    series_id INTEGER NOT NULL REFERENCES Series(id),
    uri VARCHAR NOT NULL,
    "primary" BOOLEAN DEFAULT FALSE
);

ALTER TABLE series DROP COLUMN info_link;
