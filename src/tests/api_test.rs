extern crate dotenv;

use dotenv::dotenv;


use actix_web::{
    test::{call_and_read_body_json, init_service, TestRequest, read_body_json},
    web::{Data},
    App, http::header,
};
use mongodb::{bson::Bson};
use serde::*;

use crate::{api::user_api::{create_user, get_user, get_all_users, update_user}, models::user_model::User, repository::mongodb_repo::MongoRepo};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct InsertOne {
    inserted_id: Bson,
}

#[actix_web::test]
async fn test() {    
    dotenv().ok();
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);

    let app = init_service(
        App::new()
            .app_data(db_data.clone())
            .service(create_user)
            .service(get_user)
            .service(update_user)
            .service(get_all_users),
    )
    .await;

    let user = User {
        id: None,
        name: "Jane".into(),
        location: "Sample".into(),
        title: "engineer".into()
    };

    let req = TestRequest::post()
        .uri("/user")
        .insert_header(header::ContentType::json())
        .set_json(&user)
        .send_request(&app).await;

    assert!(req.status().is_success());
    //let response = call_and_read_body(&app, req).await;

    let body: InsertOne = read_body_json(req).await;
    println!("Body of req {}", body.inserted_id.as_object_id().unwrap().to_string().trim_matches('"'));

    let req = TestRequest::get()
        .uri(&format!("/user/{}", &body.inserted_id.as_object_id().unwrap().to_string().trim_matches('"')))
        .to_request();

    let response: User = call_and_read_body_json(&app, req).await;
    assert_eq!(response.name, user.name);
}