use async_trait::async_trait;
use reqwest;
use std::sync::Arc;

mod error;
pub mod model;

#[async_trait]
pub trait Datasource {
    async  fn get_certs(&self, domain: &str) -> Result<model::Certs, error::Error>;
}

pub struct CrtShDatasource {
    http_client: Arc<reqwest::Client>,
    base_url: String
}

impl CrtShDatasource {
    pub fn new(http_client: Arc<reqwest::Client>) -> Self {
        Self {
            http_client,
            base_url: String::from("https://crt.sh")
        }
    }
}

#[async_trait]
impl Datasource for CrtShDatasource {
    async fn get_certs(&self, domain: &str) -> Result<model::Certs, error::Error> {
        let url = format!("{}/?q={}&output=json", self.base_url, domain);
        let response = self.http_client.get(url)
            .send()
            .await;

        let data = match response {
            Ok(response) => {
                response.json::<model::Certs>().await
            },
            Err(_) => return Err(error::Error::RequestError)
        };

        match data {
            Ok(data) => Ok(data),
            Err(_) => Err(error::Error::ParseError)
        }
    }
}