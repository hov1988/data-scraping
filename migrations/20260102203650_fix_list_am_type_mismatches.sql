-- ================================
-- FIX external_id type
-- ================================
ALTER TABLE houses_data.list_am_houses
    ALTER COLUMN external_id TYPE TEXT
    USING external_id::TEXT;

-- ================================
-- FIX numeric counters
-- ================================
ALTER TABLE houses_data.list_am_houses
    ALTER COLUMN rooms TYPE SMALLINT
        USING rooms::SMALLINT,
    ALTER COLUMN floors TYPE SMALLINT
        USING floors::SMALLINT,
    ALTER COLUMN bathrooms TYPE SMALLINT
        USING bathrooms::SMALLINT;

-- ================================
-- FIX area fields (m²)
-- ================================
ALTER TABLE houses_data.list_am_houses
    ALTER COLUMN house_area_m2 TYPE REAL
        USING house_area_m2::REAL,
    ALTER COLUMN land_area_m2 TYPE REAL
        USING land_area_m2::REAL;

-- ================================
-- FIX timestamps
-- ================================
ALTER TABLE houses_data.list_am_houses
    ALTER COLUMN created_at TYPE TIMESTAMPTZ
        USING created_at::TIMESTAMPTZ,
    ALTER COLUMN updated_at TYPE TIMESTAMPTZ
        USING updated_at::TIMESTAMPTZ;

-- ================================
-- FIX price history date
-- ================================
ALTER TABLE houses_data.list_am_price_history
    ALTER COLUMN date TYPE TIMESTAMPTZ
        USING date::TIMESTAMPTZ;
