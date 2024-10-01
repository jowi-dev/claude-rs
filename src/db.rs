use sea_orm::{ConnectOptions, Database, DatabaseConnection};


// make this an env that key's off the nix config
const DATABASE_URL: &str = "sqlite://test.db?mode=rwc";

pub async fn connect() -> Result<DatabaseConnection, Box<dyn std::error::Error>> {
    // any other options worth adding?
    // See: https://www.sea-ql.org/SeaORM/docs/install-and-config/connection/
    let opts = ConnectOptions::new(DATABASE_URL);


    return Ok(Database::connect(opts).await?);
}

pub async fn disconnect(conn: DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    //probably some logging here in the future

    conn.close().await?;

    return Ok(())
}
