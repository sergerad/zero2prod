use actix_web::{web, HttpRequest, HttpResponse, Responder};
use sqlx::PgPool;
#[derive(serde::Deserialize, Debug)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}

pub async fn subscribe(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        uuid::Uuid::new_v4(),
        form.email,
        form.name,
        chrono::Utc::now()
    )
    .execute(pool.get_ref())
    .await
    {
        Err(err) => {
            log::error!(
                "Failed to insert {form:?} into subscriptions: {err:?}"
            );
            HttpResponse::InternalServerError().finish()
        }
        Ok(_) => HttpResponse::Ok().finish(),
    }
}
