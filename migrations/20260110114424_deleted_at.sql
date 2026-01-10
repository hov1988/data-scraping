-- Add migration script here
ALTER TABLE houses_data.list_am_houses
    ADD COLUMN deleted_at timestamp with time zone;
