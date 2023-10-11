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

    pub async fn fetch_discordid(&self, steamid: i64) -> Result<u64, Error> {
        let row = self.client.query_one(
            "SELECT discordid FROM steamids WHERE steamid = $1",
            &[&steamid],
        ).await?;

        let discordid: f64 = row.get(0);

        Ok(discordid as u64)
    }

    //pub async fn insert_ids(&self, steamid: String, discordid: String) -> Result<(), Error> {
    //    self.client.execute(
    //        "INSERT INTO steamid (steamid, discordid) VALUES ($1, $2)",
    //        &[&steamid, &discordid],
    //    ).await?;

    //    Ok(())
    //}
}