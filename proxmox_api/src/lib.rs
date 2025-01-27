use anyhow;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};

use kernel::error::{ProxmoxApiError, ProxmoxApiResult};

#[derive(Debug, Deserialize)]
pub struct VersionInfo {
    pub release: String,
    pub repoid: String,
    pub version: String,
}

#[derive(Serialize)]
pub struct CreateVirtualMachineRequest {
    /// 仮想マシンのユニークなID
    pub vmid: u32,
    /// 仮想マシンの名前
    pub name: String,
    /// 割り当てるメモリのサイズ
    pub memory: u64,
    /// 割り当てるCPUコア数
    pub cores: u32,
    /// ストレージの設定
    pub ide0: String,
    /// ネットワークの設定
    pub net0: String,
    /// OSタイプの指定
    pub ostype: String,
}

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
        body: &CreateVirtualMachineRequest,
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

    pub async fn get_version(&self) -> ProxmoxApiResult<VersionInfo> {
        match self.get("/version").await {
            Ok(response) => {
                if response.status().is_success() {
                    let json: VersionInfo = response
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
