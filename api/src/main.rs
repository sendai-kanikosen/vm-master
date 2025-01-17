use axum::{http::StatusCode, routing::get, Router};
use serde::{Deserialize, Serialize};
use std::net::{Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() {
    let hc_router = Router::new().route("/healthz", get(healthcheck));
    //swaggerUIをルートに統合->swagger-uiをアプリケーションに追加する->APIドキュメント見れる
    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .nest("/hc", hc_router);
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", addr);

    axum::serve(listener, app).await.unwrap();
}
//その関数が提供するAPIのエンドポイントに関する情報をOpenAPIドキュメントに記述する
#[utoipa::path(
    get,
    path = "/healthz",
    responses(
        (status = 200, description = "Health check succesfully",
        body = HealthCheck),
    ),
    tag = "HealthCheck",
)]
pub async fn healthcheck() -> (StatusCode, axum::Json<HealthCheck>) {
    (StatusCode::OK, axum::Json(HealthCheck { done: true }))
} //謎：fn healthcheckと#[utoipa::path()]を入れ替えると、エラーを吐き出す
#[derive(Serialize, Deserialize, ToSchema)]
pub struct HealthCheck {
    done: bool,
}

//OpenAPIドキュメント全体を構築するよ。詳細はNotion
#[derive(OpenApi)]
#[openapi(
    paths(healthcheck),
    components(schemas(HealthCheck)),
    tags((name = "HealthCheck", description = "Health check endpoints")),
    info(
        title = "HealthCheck",
        version = "1.0.0",
    )
)]
pub struct ApiDoc;
