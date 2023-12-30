#[tokio::main]
async fn main() -> Result<(), lambda_http::Error> {
    tracing_subscriber::fmt() // required to enable CloudWatch error logging by the runtime
        .with_max_level(tracing::Level::INFO)
        .with_target(false) // disable printing the name of the module in every log line.
        .without_time() // disabling time is handy because Cloudwatch will add the ingestion time.
        .init();

    tracing::info!("🚀 Starting server on http://localhost:9000");

    lambda_http::run(app()).await
}

fn app() -> axum::Router {
    axum::Router::new()
        .route("/", axum::routing::get(ping))
        .route("/ping", axum::routing::get(ping))
}

async fn ping() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({ "ok": true }))
}

#[cfg(test)]
mod tests {
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    use super::*;

    #[tokio::test]
    async fn ping() {
        let app = app();

        let resp = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/ping")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), axum::http::StatusCode::OK);

        let body = resp.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"{\"ok\":true}");
    }
}
