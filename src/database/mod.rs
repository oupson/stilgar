use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::message::Message;

pub(crate) async fn open_connection(url: &str) -> anyhow::Result<PgPool> {
    let pool = PgPoolOptions::new().max_connections(5).connect(url).await?;

    sqlx::migrate!().run(&pool).await?;

    Ok(pool)
}

pub(crate) async fn insert_record(pool: &PgPool, msg: &Message) -> anyhow::Result<()> {
    let record = sqlx::query!(
        "select s.sensormacadresse, li.locatedinroomid as \"roomId?\" from sensor s left join located_in li on s.sensormacadresse = li.locatedinsensormacadresse where s.sensormacadresse = $1",
        msg.mac()
    )
    .fetch_optional(pool)
    .await?;

    if let Some(record) = record {
        tracing::trace!("Sensor already known");

        if let Some(room_id) = record.roomId {
            tracing::trace!("Room id is {}", room_id);
            sqlx::query!(
                "INSERT INTO RECORD(recordRoomId, recordDateTime, recordTemperature, recordHumidity) VALUES($1, now() , $2, $3)",
                room_id,
                msg.temperature(),
                msg.humidity()
            )
            .execute(pool)
            .await?;
        } else {
            tracing::trace!("Not attached to a room")
        }
    } else {
        tracing::trace!("Inserting sensor");
        sqlx::query!(
            "INSERT INTO SENSOR(sensorMacAdresse, sensorName) VALUES($1, $2)",
            msg.mac(),
            None::<&str>
        )
        .execute(pool)
        .await?;
    }

    Ok(())
}
