-- Add migration script here
-- Create table for multiple phone numbers per house (List.am)

CREATE TABLE IF NOT EXISTS houses_data.list_am_phones (
    id BIGSERIAL PRIMARY KEY,

    house_id BIGINT NOT NULL
        REFERENCES houses_data.list_am_houses(id)
        ON DELETE CASCADE,

    raw TEXT NOT NULL,       -- e.g. 091071996
    display TEXT NOT NULL,   -- e.g. (091) 07-19-96
    source TEXT NOT NULL,    -- direct | viber | whatsapp

    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    CONSTRAINT uq_list_am_phones_house_raw
        UNIQUE (house_id, raw)
);

-- Helpful indexes
CREATE INDEX IF NOT EXISTS idx_list_am_phones_house_id
    ON houses_data.list_am_phones (house_id);

CREATE INDEX IF NOT EXISTS idx_list_am_phones_source
    ON houses_data.list_am_phones (source);
