use tokio_postgres::Error;

// steamid struct which has an insert and retrieve function
pub struct SteamID {
    pub steamid: i64,
    pub discordid: i64,
}

impl Steamid {
    pub async fn insert(&self, database: &Database) -> Result<(), Error> {
        match database.insert_ids(self.steamid, self.discordid).await {
            Ok(_) => Ok(()),
            Err(e) => {
                dbg!(e);
                Err(Error::DatabaseError)
            }
        }
    }

    pub async fn retrieve(&self, database: &Database) -> Result<i64, Error> {
        match database.fetch_steamid(self.discordid).await {
            Ok(steamid) => Ok(steamid),
            Err(e) => {
                dbg!(e);
                Err(Error::DatabaseError)
            }
        }
    }
}

pub struct Score {
    pub steamid: i64,
    pub map: String,
    pub car: Option<String>,
    pub score: Option<i64>,
}
