-- Add migration script here
CREATE TABLE houses_data.list_am_phones (
    id BIGSERIAL PRIMARY KEY,
    house_id BIGINT NOT NULL
        REFERENCES houses_data.list_am_houses(id)
        ON DELETE CASCADE,

    raw TEXT NOT NULL,
    display TEXT NOT NULL,
    source TEXT NOT NULL, -- direct | viber | whatsapp

    UNIQUE (house_id, raw)
);
