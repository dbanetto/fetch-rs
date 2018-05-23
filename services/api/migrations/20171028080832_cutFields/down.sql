ALTER TABLE series ADD COLUMN start_date DATE;
ALTER TABLE series ADD COLUMN end_date DATE;
ALTER TABLE series ADD COLUMN episodes_total INTEGER;
ALTER TABLE series ADD COLUMN episodes_current INTEGER NOT NULL CHECK (episodes_current >= 0) DEFAULT 0;
ALTER TABLE series ADD CONSTRAINT episode_check CHECK (episodes_total >= episodes_current);
