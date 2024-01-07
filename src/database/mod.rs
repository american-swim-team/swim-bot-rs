pub mod config;

use tokio_postgres::{NoTls, Error, Client};

#[derive(Debug)]
pub struct Database {
    client: Client,
}

impl Database {
    pub async fn new(config: config::DatabaseConfig) -> Result<Database, Error> {
        let (client, connection) = tokio_postgres::connect(
            &format!("host={} user={} password={} dbname={} port={}",
                config.address, config.username, config.password, config.database, config.port),
            NoTls,
        ).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Error occured while accessing the database: \n {}", e);
            }
        });

        Ok(Database { client })
    }

    pub async fn query_one(&self, query: &str, params: &[&(dyn tokio_postgres::types::ToSql + Sync)]) -> Result<tokio_postgres::Row, Error> {
        let row = self.client.query_one(query, params).await?;
        Ok(row)
    }

    pub async fn query(&self, query: &str, params: &[&(dyn tokio_postgres::types::ToSql + Sync)]) -> Result<Vec<tokio_postgres::Row>, Error> {
        let rows = self.client.query(query, params).await?;
        Ok(rows)
    }

    pub async fn execute(&self, query: &str, params: &[&(dyn tokio_postgres::types::ToSql + Sync)]) -> Result<(), Error> {
        self.client.execute(query, params).await?;
        Ok(())
    }
}