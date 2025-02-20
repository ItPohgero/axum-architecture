use sqlx::{Connection, PgConnection};

#[tokio::test]
async fn test_database_connection() -> Result<(), sqlx::Error> {
    let url = "postgres://asani:root@localhost:5432/rust";
    let connection = PgConnection::connect(url).await?;
    connection.close().await?;
    Ok(())
}