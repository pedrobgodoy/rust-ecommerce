use std::sync::Arc;

use actix_web::{
    error, get,
    http::{header::ContentType, StatusCode},
    post, web, App, HttpResponse, HttpServer, Responder,
};
use bigdecimal::{BigDecimal, FromPrimitive};
use derive_more::{Display, Error};
use serde::Deserialize;

use crate::{
    domain::{
        commands::{self, Command},
        queries::Query,
        service::ApplicationService,
    },
    infra,
};

#[derive(Deserialize)]
struct CreateItemInput {
    name: String,
    description: String,
    price: f32,
    image_url: String,
}

#[derive(Debug, Display, Error)]
enum MyError {
    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "bad request")]
    BadClientData,

    #[display(fmt = "timeout")]
    Timeout,
}

impl error::ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            MyError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::BadClientData => StatusCode::BAD_REQUEST,
            MyError::Timeout => StatusCode::GATEWAY_TIMEOUT,
        }
    }
}

#[post("/items")]
async fn create_item(
    input: web::Json<CreateItemInput>,
    app_service: web::Data<ApplicationService>,
) -> Result<&'static str, MyError> {
    let create_item_cmd = commands::CreateItem::new(
        input.name.clone(),
        input.description.clone(),
        BigDecimal::from_f32(input.price).unwrap(),
        input.image_url.clone(),
    );
    let result = app_service.create_item.handle(create_item_cmd).await;
    match result {
        Ok(_) => Ok("OK"),
        Err(_) => Err(MyError::InternalError),
    }
}

#[get("/items/{id}")]
async fn get_item_by_id(
    path: web::Path<String>,
    app_service: web::Data<ApplicationService>,
) -> impl Responder {
    let id = path.into_inner();
    let query = crate::domain::queries::GetItem::new(id);
    let items = app_service.get_item.handle(query).await;
    format!("{:?}", items)
}

pub async fn setup() -> std::io::Result<()> {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(500)
        .connect("postgres://postgres:example@localhost:5432/catalog")
        .await
        .unwrap();

    println!("Starting server at http://0.0.0.0:8080");

    HttpServer::new(move || {
        let item_repo = Arc::new(infra::repositories::SqlxItemRepository::new(pool.clone()));
        let application_service = Arc::new(ApplicationService::new(item_repo));
        App::new()
            .service(get_item_by_id)
            .service(create_item)
            .app_data(web::Data::from(application_service))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
