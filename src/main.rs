use infra::http;

mod domain;
mod infra;

#[tokio::main]
async fn main() {
    http::setup().await.unwrap();
}
