use actix_web::web::ServiceConfig;
use migration::{Migrator, MigratorTrait};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_secrets::SecretStore;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let jwt_secret = secret_store
        .get("JWT_SECRET")
        .expect("Could not find required secret JWT_SECRET");
    let db_url = secret_store
        .get("DB_URL")
        .expect("Could not find required secret DB_URL");

    execute_migrations(&db_url)
        .await
        .expect("Failed to execute migrations");

    Ok(api::main(jwt_secret).into())
}

async fn execute_migrations(database: &str) -> Result<(), migration::DbErr> {
    let connection = sea_orm::Database::connect(&*database).await?;
    Ok(Migrator::up(&connection, None).await?)
}
