use actix_web::{post, web, HttpResponse, Responder};
use chrono::Utc;
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct SubscribeData {
    email: String,
    name: String,
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool),
    fields(
        subsriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
#[post("/subscribe")]
pub async fn subscribe(form: web::Form<SubscribeData>, pool: web::Data<PgPool>) -> impl Responder {
    match insert_subscriber(&pool, &form).await {
        Ok(_) => HttpResponse::Ok(),
        Err(e) => {
            tracing::error!("Failed to execute query {:?}", e);
            HttpResponse::InternalServerError()
        }
    }
    .await
}

#[tracing::instrument(name = "Saving new subscriber details in database", skip(pool, form))]
async fn insert_subscriber(pool: &PgPool, form: &SubscribeData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at) 
        VALUES ($1, $2, $3, $4)
        "#,
        uuid::Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now(),
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
