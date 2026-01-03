-- Add unique indexes (sqlx-compatible, transactional)

/* ============================================================
   list_am_features
   Enforce uniqueness per house + feature + value
   ============================================================ */

CREATE UNIQUE INDEX IF NOT EXISTS
    idx_list_am_features_house_feature_value_unique
ON houses_data.list_am_features (house_id, feature_type, value);


/* ============================================================
   list_am_price_history
   Enforce uniqueness per house + date + price + diff
   ============================================================ */

CREATE UNIQUE INDEX IF NOT EXISTS
    idx_list_am_price_history_house_date_price_diff_unique
ON houses_data.list_am_price_history (house_id, date, price, diff);


/* ============================================================
   list_am_phones
   Enforce one phone per source per house
   (existing UNIQUE (house_id, raw) stays intact)
   ============================================================ */

CREATE UNIQUE INDEX IF NOT EXISTS
    idx_list_am_phones_house_source_unique
ON houses_data.list_am_phones (house_id, source);
