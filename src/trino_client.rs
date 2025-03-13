use trino_rust_client::{Client, ClientBuilder, Trino};
use url::Url;

pub struct TrinoClient {
    client: Client,
    query_spliter: String,
}

impl TrinoClient {

    pub fn new(connection_string: &str, query_spliter: &str) -> anyhow::Result<TrinoClient> {

        let url = Url::parse(connection_string)?;
        
        let username = match url.username() {
            "" => "trino",
            username => username
        };

        let host = url.host_str().unwrap_or("localhost");
        
        let client_builder = ClientBuilder::new(username, host);

        let client_builder = 
            if let Some(port) = url.port() {
                client_builder.port(port)
            } else {
                client_builder
            };
        
        let client_builder =
            if let Some(catalog) = url.path().strip_prefix("/") {
                if catalog.contains("/") {
                    anyhow::bail!("Catalog must be just an identifier, invalid character '/'")
                }

                client_builder.catalog(catalog)
            } else {
                client_builder
            };

        let client = client_builder.build()?;

        let trino_client = TrinoClient{
            client,
            query_spliter: query_spliter.to_owned()
        };

        Ok(trino_client)
    }

    pub async fn run_sql<S>(&self, sql: &str) -> anyhow::Result<()> where S: Trino + 'static {
        let sqls: Vec<_> = sql.split(&self.query_spliter).collect();

        for sql in sqls {
            self.client.execute(sql.to_owned()).await?;
        }

        Ok(())
    }
}