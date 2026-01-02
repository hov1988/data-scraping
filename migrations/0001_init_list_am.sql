-- Schema
CREATE SCHEMA IF NOT EXISTS houses_data;

-- ============================
-- Houses (List.am)
-- ============================
CREATE TABLE IF NOT EXISTS houses_data.list_am_houses (
    id BIGSERIAL PRIMARY KEY,

    external_id BIGINT NOT NULL UNIQUE,
    url TEXT NOT NULL,

    condition TEXT,
    rooms INTEGER,
    house_area_m2 INTEGER,
    land_area_m2 INTEGER,
    floors INTEGER,
    bathrooms INTEGER,

    construction_type TEXT,
    renovation TEXT,
    garage TEXT,
    furniture TEXT,

    description TEXT,
    location TEXT,

    seller_name TEXT,
    phone TEXT,

    created_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ,

    scraped_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- ============================
-- Price history
-- ============================
CREATE TABLE IF NOT EXISTS houses_data.list_am_price_history (
    id BIGSERIAL PRIMARY KEY,
    house_id BIGINT NOT NULL
        REFERENCES houses_data.list_am_houses(id)
        ON DELETE CASCADE,

    date DATE NOT NULL,
    price TEXT NOT NULL,
    diff TEXT
);

-- ============================
-- Images
-- ============================
CREATE TABLE IF NOT EXISTS houses_data.list_am_images (
    id BIGSERIAL PRIMARY KEY,
    house_id BIGINT NOT NULL
        REFERENCES houses_data.list_am_houses(id)
        ON DELETE CASCADE,

    position INTEGER NOT NULL,
    url TEXT NOT NULL
);

-- ============================
-- Features
-- ============================
CREATE TABLE IF NOT EXISTS houses_data.list_am_features (
    id BIGSERIAL PRIMARY KEY,
    house_id BIGINT NOT NULL
        REFERENCES houses_data.list_am_houses(id)
        ON DELETE CASCADE,

    feature_type TEXT NOT NULL,
    value TEXT NOT NULL
);

-- ============================
-- Indexes
-- ============================
CREATE INDEX IF NOT EXISTS idx_list_am_houses_external_id
    ON houses_data.list_am_houses(external_id);

CREATE INDEX IF NOT EXISTS idx_list_am_price_history_house_id
    ON houses_data.list_am_price_history(house_id);

CREATE INDEX IF NOT EXISTS idx_list_am_images_house_id
    ON houses_data.list_am_images(house_id);

CREATE INDEX IF NOT EXISTS idx_list_am_features_house_id
    ON houses_data.list_am_features(house_id);
