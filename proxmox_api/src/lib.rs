use anyhow;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};

use kernel::error::{ProxmoxApiError, ProxmoxApiResult};
use proxmox_api::nodes::node::qemu::PostParams;
use proxmox_api::version::GetOutput;

pub struct ProxmoxAPIClient {
    client: Client,
    base_url: String,
    token: String,
}

impl ProxmoxAPIClient {
    pub fn new(base_url: &str, token: &str) -> Self {
        let client = Client::new();
        Self {
            client,
            base_url: base_url.to_string(),
            token: token.to_string(),
        }
    }

    async fn get(&self, path: &str) -> reqwest::Result<Response> {
        let url = format!("{}{}", self.base_url, path);
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&self.token).unwrap());
        let response = self.client.get(url).headers(headers).send().await?;
        Ok(response)
    }

    pub async fn post<T: Serialize>(&self, path: &str, body: &T) -> reqwest::Result<Response> {
        let url = format!("{}{}", self.base_url, path);

        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&self.token).unwrap());
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );

        let response = self
            .client
            .post(url)
            .headers(headers)
            .json(body)
            .send()
            .await?;

        Ok(response)
    }

    pub async fn create_virtual_machine(
        &self,
        node: &str,
        body: &PostParams,
    ) -> ProxmoxApiResult<Response> {
        let path = format!("/nodes/{}/qemu", node);

        match self.post(&path, body).await {
            Ok(response) => {
                if response.status().is_success() {
                    Ok(response)
                } else {
                    let status = response.status();
                    let error_text = response
                        .text()
                        .await
                        .unwrap_or_else(|_| "No response body".to_string());
                    Err(ProxmoxApiError(anyhow::anyhow!(
                        "Request failed with status {}: {}",
                        status,
                        error_text
                    )))
                }
            }
            Err(err) => Err(ProxmoxApiError(anyhow::anyhow!(
                "Failed to send request: {}",
                err
            ))),
        }
    }

    pub async fn get_version(&self) -> ProxmoxApiResult<GetOutput> {
        match self.get("/version").await {
            Ok(response) => {
                if response.status().is_success() {
                    let json: GetOutput = response
                        .json()
                        .await
                        .map_err(ProxmoxApiWrapperError::from)?;
                    Ok(json)
                } else {
                    let status = response.status();
                    let error_text = response
                        .text()
                        .await
                        .unwrap_or_else(|_| "No response body".to_string());
                    Err(ProxmoxApiError(anyhow::anyhow!(
                        "Request failed with status {}: {}",
                        status,
                        error_text
                    )))
                }
            }
            Err(err) => Err(ProxmoxApiError(anyhow::anyhow!(
                "Failed to send request: {}",
                err
            ))),
        }
    }
}

pub struct ProxmoxApiWrapperError(pub anyhow::Error);

impl From<reqwest::Error> for ProxmoxApiWrapperError {
    fn from(err: reqwest::Error) -> Self {
        ProxmoxApiWrapperError(anyhow::Error::new(err))
    }
}

impl From<ProxmoxApiWrapperError> for ProxmoxApiError {
    fn from(err: ProxmoxApiWrapperError) -> Self {
        ProxmoxApiError(err.0)
    }
}
