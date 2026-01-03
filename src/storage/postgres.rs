use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::{
    postgres::PgPoolOptions,
    PgPool,
    Postgres,
    Transaction,
};

use crate::crawler::models::HouseDetails;

pub struct Storage {
    pool: PgPool,
}

impl Storage {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        Ok(Self { pool })
    }

    pub async fn save_houses_batch(
        &self,
        houses: &[HouseDetails],
    ) -> Result<usize> {
        let mut tx = self.pool.begin().await?;
        let mut saved = 0usize;

        for house in houses {
            self.save_house_tx(&mut tx, house).await?;
            saved += 1;
        }

        tx.commit().await?;
        Ok(saved)
    }

    pub async fn save_house(&self, house: &HouseDetails) -> Result<i64> {
        let mut tx = self.pool.begin().await?;
        let id = self.save_house_tx(&mut tx, house).await?;
        tx.commit().await?;
        Ok(id)
    }

    async fn save_house_tx(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        house: &HouseDetails,
    ) -> Result<i64> {

        let house_id = sqlx::query!(
            r#"
            INSERT INTO houses_data.list_am_houses (
                external_id,
                url,
                title,
                price,
                seller_name,
                condition,
                rooms,
                house_area_m2,
                land_area_m2,
                construction_type,
                floors,
                bathrooms,
                garage,
                renovation,
                furniture,
                description,
                location,
                amenities,
                comfort,
                ceiling_height,
                prepayment,
                utility_payments,
                lease_type,
                minimum_rental_period,
                sewerage,
                parking,
                entrance,
                location_from_street,
                elevator,
                floor_area,
                created_at,
                updated_at
            )
            VALUES (
                $1,$2,$3,$4,
                $5,
                $6,$7,$8,$9,$10,$11,$12,$13,$14,$15,
                $16,$17,
                $18,$19,$20,$21,$22,$23,$24,$25,$26,$27,$28,$29,$30,
                $31,$32
            )
            ON CONFLICT (external_id) DO UPDATE SET
                title = EXCLUDED.title,
                price = EXCLUDED.price,
                seller_name = EXCLUDED.seller_name,
                condition = EXCLUDED.condition,
                rooms = EXCLUDED.rooms,
                house_area_m2 = EXCLUDED.house_area_m2,
                land_area_m2 = EXCLUDED.land_area_m2,
                construction_type = EXCLUDED.construction_type,
                floors = EXCLUDED.floors,
                bathrooms = EXCLUDED.bathrooms,
                garage = EXCLUDED.garage,
                renovation = EXCLUDED.renovation,
                furniture = EXCLUDED.furniture,
                description = EXCLUDED.description,
                location = EXCLUDED.location,
                amenities = EXCLUDED.amenities,
                comfort = EXCLUDED.comfort,
                ceiling_height = EXCLUDED.ceiling_height,
                prepayment = EXCLUDED.prepayment,
                utility_payments = EXCLUDED.utility_payments,
                lease_type = EXCLUDED.lease_type,
                minimum_rental_period = EXCLUDED.minimum_rental_period,
                sewerage = EXCLUDED.sewerage,
                parking = EXCLUDED.parking,
                entrance = EXCLUDED.entrance,
                location_from_street = EXCLUDED.location_from_street,
                elevator = EXCLUDED.elevator,
                floor_area = EXCLUDED.floor_area,
                updated_at = EXCLUDED.updated_at,
                scraped_at = now()
            RETURNING id
            "#,
            house.external_id,
            house.url,
            house.title,
            house.price,
            house.contact.seller_name,
            house.condition,
            house.rooms.map(|v| v as i16),
            house.house_area_m2,
            house.land_area_m2,
            house.construction_type,
            house.floors.map(|v| v as i16),
            house.bathrooms.map(|v| v as i16),
            house.garage,
            house.renovation,
            house.furniture,
            house.description,
            house.location,
            house.amenities,
            house.comfort,
            house.ceiling_height,
            house.prepayment,
            house.utility_payments,
            house.lease_type,
            house.minimum_rental_period,
            house.sewerage,
            house.parking,
            house.entrance,
            house.location_from_street,
            house.elevator,
            house.floor_area,
            parse_iso(&house.created_at),
            parse_iso(&house.updated_at)
        )
        .fetch_one(&mut **tx)
        .await?
        .id;        

        // Phones
        for phone in &house.contact.phones {
            sqlx::query!(
                r#"
                INSERT INTO houses_data.list_am_phones
                    (house_id, raw, display, source)
                VALUES ($1, $2, $3, $4)
                ON CONFLICT (house_id, source)
                DO UPDATE SET
                    raw = EXCLUDED.raw,
                    display = EXCLUDED.display
                "#,
                house_id,
                phone.raw,
                phone.display,
                phone.source
            )
            .execute(&mut **tx) 
            .await?;
        }

        // Price history
        for p in &house.price_history {
            let date = DateTime::parse_from_rfc3339(&p.date)
                .map(|dt| dt.with_timezone(&Utc))
                .ok();

            sqlx::query!(
                r#"
                INSERT INTO houses_data.list_am_price_history
                    (house_id, date, price, diff)
                VALUES ($1, $2, $3, $4)
                ON CONFLICT (house_id, date, price, diff) DO NOTHING
                "#,
                house_id,
                date,
                p.price,
                p.diff
            )
            .execute(&mut **tx)
            .await?;
        }

        // Images
        for (pos, url) in house.images.iter().enumerate() {
            sqlx::query!(
                r#"
                INSERT INTO houses_data.list_am_images
                    (house_id, position, url)
                VALUES ($1, $2, $3)
                ON CONFLICT DO NOTHING
                "#,
                house_id,
                pos as i32,
                url
            )
            .execute(&mut **tx)
            .await?;
        }

        // Features
        Self::insert_features(tx, house_id, "appliances", &house.appliances).await?;
        Self::insert_features(tx, house_id, "service_lines", &house.service_lines).await?;
        Self::insert_features(tx, house_id, "facilities", &house.facilities).await?;

        Ok(house_id)
    }

    async fn insert_features(
        tx: &mut Transaction<'_, Postgres>,
        house_id: i64,
        feature_type: &str,
        values: &[String],
    ) -> Result<()> {
        for v in values {
            sqlx::query!(
                r#"
                INSERT INTO houses_data.list_am_features
                    (house_id, feature_type, value)
                VALUES ($1, $2, $3)
                ON CONFLICT (house_id, feature_type, value) DO NOTHING
                "#,
                house_id,
                feature_type,
                v
            )
            .execute(&mut **tx)
            .await?;
        }
        Ok(())
    }
}

fn parse_iso(ts: &Option<String>) -> Option<DateTime<Utc>> {
    ts.as_ref()
        .and_then(|v| DateTime::parse_from_rfc3339(v).ok())
        .map(|dt| dt.with_timezone(&Utc))
}