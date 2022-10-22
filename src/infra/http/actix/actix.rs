use std::sync::Arc;

use actix_web::{
    error, get,
    http::{header::ContentType, StatusCode},
    post, web, App, HttpResponse, HttpServer,
};
use bigdecimal::{BigDecimal, FromPrimitive};
use derive_more::{Display, Error};
use serde::Deserialize;

use crate::domain::{
    commands::{self, Command},
    queries::Query,
    service::ApplicationService,
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
        Err(_) => {
            println!("Error ${:?}", result);
            Err(MyError::InternalError)
        }
    }
}

#[get("/items/{id}")]
async fn get_item_by_id(
    path: web::Path<String>,
    app_service: web::Data<ApplicationService>,
) -> Result<String, MyError> {
    let id = path.into_inner();
    let query = crate::domain::queries::GetItem::new(id);
    let result = app_service.get_item.handle(query).await;
    match result {
        Ok(_) => Ok(format!("{:?}", result)),
        Err(_) => {
            println!("Error ${:?}", result);
            Err(MyError::InternalError)
        }
    }
}

pub async fn setup(app_service: Arc<ApplicationService>) -> std::io::Result<()> {
    println!("Starting server at http://0.0.0.0:8080");

    HttpServer::new(move || {
        App::new()
            .service(get_item_by_id)
            .service(create_item)
            .app_data(web::Data::from(Arc::clone(&app_service)))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
