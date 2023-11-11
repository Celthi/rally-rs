use anyhow::Result;
use tokio_postgres::{Client, NoTls};

pub struct DB {
    client: Client,
}

impl DB {
    pub async fn new(host: &str, user: &str, password: &str, port: Option<&str>) -> Result<Self> {
        let connect_string = if port.is_none() {
            format!("host={} user={} password={}", host, user, password)
        } else {
            format!(
                "host={} port={} user={} password={}",
                host,
                port.unwrap(),
                user,
                password
            )
        };

        let (client, connection) = tokio_postgres::connect(&connect_string, NoTls).await?;

        // The connection object performs the actual communication with the database,
        // so spawn it off to run on its own.
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        Ok(DB { client })
    }

    // will update the location if insert the debug id again
    pub async fn insert_rally_token(&mut self, user_name: &str, token: &str) -> Result<()> {
        self.client.execute(
            "INSERT INTO rallytokens (user_name, token) VALUES ($1, $2) ON CONFLICT (user_name) DO UPDATE set token=$3",
            &[&user_name, &token, &token],
        ).await?;
        Ok(())
    }

    pub async fn get_rally_token(&mut self, user: &str) -> Result<Option<String>> {
        let row = self
            .client
            .query_one("SELECT token FROM rallytokens where user_name=$1", &[&user])
            .await?;
        let token: &str = row.get(0);
        Ok(Some(token.to_string()))
    }
}
