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

pub struct ProxmoxAPIClient {
    client: Client,
    base_url: String,
    token: String,
}

#[derive(Serialize)]
pub struct CloneRequest {
    pub newid: u32,
    pub name: Option<String>,
    pub target: Option<String>,
    pub full: u32,
}

#[derive(Debug, Deserialize)]
pub struct CloneResponse {
    pub data: Option<String>,
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

    pub async fn get_version(&self) -> ProxmoxApiResult<VersionInfo> {
        // 非同期関数を呼び出すには `await` が必要
        match self.get("/version").await {
            Ok(response) => {
                // ステータスコードの確認
                if response.status().is_success() {
                    // JSONレスポンスをパース
                    let json: VersionInfo = response
                        .json()
                        .await
                        .map_err(ProxmoxApiWrapperError::from)?;
                    Ok(json)
                } else {
                    // エラーステータスの場合はエラーを返す
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
            Err(err) => {
                // リクエスト自体が失敗した場合のエラーハンドリング
                Err(ProxmoxApiError(anyhow::anyhow!(
                    "Failed to send request: {}",
                    err
                )))
            }
        }
    }

    pub async fn clone_vm(
        &self,
        node: &str,
        vmid: &str,
        body: CloneRequest,
    ) -> ProxmoxApiResult<CloneResponse> {
        let url = format!("{}/nodes/{}/qemu/{}/clone", self.base_url, node, vmid);
        match self.post(&url, &body).await {
            Ok(response) => {
                // ステータスコードの確認
                if response.status().is_success() {
                    // JSONレスポンスをパース
                    let json: CloneResponse = response
                        .json()
                        .await
                        .map_err(ProxmoxApiWrapperError::from)?;
                    Ok(json)
                } else {
                    // エラーステータスの場合はエラーを返す
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
            Err(err) => {
                // リクエスト自体が失敗した場合のエラーハンドリング
                Err(ProxmoxApiError(anyhow::anyhow!(
                    "Failed to send request: {}",
                    err
                )))
            }
        }
    }
}

pub struct ProxmoxApiWrapperError(pub anyhow::Error);

impl From<reqwest::Error> for ProxmoxApiWrapperError {
    fn from(err: reqwest::Error) -> Self {
        ProxmoxApiWrapperError(anyhow::Error::new(err))
    }
}

// ProxmoxApiWrapperError -> ProxmoxApiError への変換
impl From<ProxmoxApiWrapperError> for ProxmoxApiError {
    fn from(err: ProxmoxApiWrapperError) -> Self {
        ProxmoxApiError(err.0)
    }
}
