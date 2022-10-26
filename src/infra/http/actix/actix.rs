use std::sync::Arc;

use actix_web::{get, http::header::ContentType, post, web, App, HttpResponse, HttpServer};
use bigdecimal::{BigDecimal, FromPrimitive};
use serde::Deserialize;

use crate::{
    domain::{
        commands::{self, Command},
        queries::Query,
        repositories::ItemRepositoryError,
        service::ApplicationService,
    },
    infra::http::http::{HttpError, HttpOptions},
};

#[derive(Deserialize)]
struct CreateItemInput {
    name: String,
    description: String,
    price: f32,
    image_url: String,
}

#[post("/items")]
async fn create_item(
    input: web::Json<CreateItemInput>,
    app_service: web::Data<ApplicationService>,
) -> Result<&'static str, HttpError> {
    let create_item_cmd = commands::CreateItem::new(
        input.name.clone(),
        input.description.clone(),
        BigDecimal::from_f32(input.price).unwrap(),
        input.image_url.clone(),
    );
    let result = app_service.create_item.handle(create_item_cmd).await;
    match result {
        Ok(_) => Ok(""),
        Err(ItemRepositoryError::NotFound) => Err(HttpError::NotFound),
        Err(_) => {
            println!("Error ${:?}", result);
            Err(HttpError::InternalError)
        }
    }
}

#[get("/items/{id}")]
async fn get_item_by_id(
    path: web::Path<String>,
    app_service: web::Data<ApplicationService>,
) -> Result<HttpResponse, HttpError> {
    let id = path.into_inner();
    let query = crate::domain::queries::GetItem::new(id);
    let result = app_service.get_item.handle(query).await;
    match result {
        Ok(item) => Ok(HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(serde_json::to_string(&item).unwrap())),
        Err(ItemRepositoryError::NotFound) => Err(HttpError::NotFound),
        Err(_) => {
            println!("Error ${:?}", result);
            Err(HttpError::InternalError)
        }
    }
}

pub async fn setup(
    app_service: Arc<ApplicationService>,
    options: HttpOptions,
) -> std::io::Result<()> {
    println!("Starting HTTP server at {}:{}", options.host, options.port);

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
