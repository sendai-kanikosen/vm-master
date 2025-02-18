use dotenvy::dotenv;
use proxmox_api::types::VmId;
use sqlx::{database, postgres::PgPoolOptions, Pool, Postgres};
use std::env;

use kernel::model::template::Template;

pub async fn template_serch(template_name: String) -> Result<Template, sqlx::Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("Connected to the database!");

    let template_data = sqlx::query_as!(
        Template,
        r#"
        select vmid, node
        from template
        where name = $1"#,
        template_name
    )
    .fetch_one(&pool)
    .await?;

    Ok(template_data)
}
