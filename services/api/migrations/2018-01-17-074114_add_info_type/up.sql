ALTER TABLE info_blob ADD COLUMN info_type VarChar NOT NULL DEFAULT 'url';

ALTER TABLE info_blob ALTER COLUMN info_type DROP DEFAULT;
