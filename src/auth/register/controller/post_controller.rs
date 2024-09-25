use actix_web::{web, HttpResponse, Responder};
use deadpool_postgres::Pool;

use crate::{
    auth::register::{model::post_request::RegisterUserRequest, repository},
    base::model::{response::http_response::BaseHttpResponse, user_model::User},
};

pub async fn execute(
    pool: web::Data<Pool>,
    json: web::Json<RegisterUserRequest>,
) -> impl Responder {
    let client = pool.get().await.expect("Error connecting to the database");
    let result_repo = repository::create_new_user::execute(&client, &User::from(&(json.0))).await;
    match result_repo {
        Ok(user) => {
            let response = BaseHttpResponse::<User> {
                status: 200,
                success: true,
                data: user,
            };
            HttpResponse::Ok().json(response)
        }
        Err(error) => {
            println!("{:?}", error.to_string());

            //handle db error
            match error.to_string().as_str() {
                err_msg
                    if err_msg.contains(
                        "duplicate key value violates unique constraint \"user_email_key\"",
                    ) =>
                {
                    let response = BaseHttpResponse::<String> {
                        status: 400,
                        success: false,
                        data: "Email already exists".to_string(),
                    };
                    HttpResponse::BadRequest().json(response)
                }
                err_msg
                    if err_msg.contains(
                        "duplicate key value violates unique constraint \"user_username_key\"",
                    ) =>
                {
                    let response = BaseHttpResponse::<String> {
                        status: 400,
                        success: false,
                        data: "Username already exists".to_string(),
                    };
                    HttpResponse::BadRequest().json(response)
                }
                _ => {
                    let response = BaseHttpResponse::<String> {
                        status: 500,
                        success: false,
                        data: "Error creating user".to_string(),
                    };
                    HttpResponse::InternalServerError().json(response)
                }
            }
        }
    }
}
