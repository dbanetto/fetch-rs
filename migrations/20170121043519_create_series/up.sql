CREATE TABLE series (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    start_date DATE,
    end_date DATE,
    episodes_total INTEGER,
    episodes_current INTEGER NOT NULL CHECK (episodes_current >= 0),
    info_link VARCHAR,
    CHECK (episodes_total >= episodes_current)
);
