use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct VersionInfo {
    release: String,
    repoid: String,
    version: String,
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    data: VersionInfo,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Proxmoxサーバーの情報
    let base_url = "https://<proxmox-server>:8006/api2/json/version";
    let token = "PVEAPIToken=root@pam!mytokenid=mysupersecrettoken";

    // HTTPクライアントを作成
    let client = Client::builder()
        .danger_accept_invalid_certs(true) // 開発環境向け: SSL証明書検証を無効化
        .build()?;

    // ヘッダーを設定
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_str(token)?);

    // GETリクエストを送信
    let response = client.get(base_url).headers(headers).send()?;

    // レスポンスの確認
    if response.status().is_success() {
        // JSONレスポンスをパース
        let json: ApiResponse = response.json()?;
        println!("Proxmox VE Version Info:");
        println!("Release: {}", json.data.release);
        println!("Repo ID: {}", json.data.repoid);
        println!("Version: {}", json.data.version);
    } else {
        eprintln!(
            "Failed to fetch version info: {} - {}",
            response.status(),
            response
                .text()
                .unwrap_or_else(|_| "No response body".to_string())
        );
    }

    Ok(())
}
