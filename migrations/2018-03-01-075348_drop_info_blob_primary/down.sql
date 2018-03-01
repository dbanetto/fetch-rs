-- This file should undo anything in `up.sql`
ALTER TABLE info_blob ADD COLUMN "primay" DEFAULT FALSE NOT NULL;
