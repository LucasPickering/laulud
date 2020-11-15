use crate::config::LauludConfig;
use mongodb::{options::ClientOptions, Client, Collection, Database};
use strum::IntoStaticStr;

const DATABASE_NAME: &str = "laulud";

#[derive(Copy, Clone, Debug, IntoStaticStr)]
pub enum CollectionName {
    #[strum(to_string = "tracks")]
    Tracks,
}

pub struct DbHandler {
    client: Client,
}

impl DbHandler {
    pub async fn connect(
        config: &LauludConfig,
    ) -> mongodb::error::Result<Self> {
        let options = ClientOptions::parse(&config.database_url).await?;
        let client = Client::with_options(options).unwrap();
        Ok(Self { client })
    }

    fn database(&self) -> Database {
        self.client.database(DATABASE_NAME)
    }

    pub fn collection(&self, collection_name: CollectionName) -> Collection {
        self.database().collection(dbg!(collection_name.into()))
    }
}
