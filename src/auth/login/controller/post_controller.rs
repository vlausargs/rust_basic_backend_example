use actix_web::{web, HttpResponse, Responder};
use bcrypt::verify;
use deadpool_postgres::Pool;

use crate::{
    auth::login::{model::post_request::AuthLoginRequest, repository::find_user_by_username},
    base::model::response::http_response::BaseHttpResponse,
};

pub async fn execute(pool: web::Data<Pool>, json: web::Json<AuthLoginRequest>) -> impl Responder {
    let client = pool.get().await.expect("Error connecting to the database");

    let users = find_user_by_username::execute(&client, &json.username).await;

    match users {
        Ok(users) => {
            let valid = verify(
                &(json.password),
                &(users.get(0).unwrap().password.as_ref().unwrap()),
            )
            .unwrap();
            match valid {
                true => {
                    let response = BaseHttpResponse::<String> {
                        status: 200,
                        success: true,
                        data: "Login Success".to_string(),
                    };
                    HttpResponse::Ok().json(response)
                }
                false => {
                    let response = BaseHttpResponse::<String> {
                        status: 500,
                        success: false,
                        data: "Username Or Password is wrong".to_string(),
                    };
                    HttpResponse::InternalServerError().json(response)
                }
            }
        }
        Err(error) => match error {
            io_error if io_error.kind() == std::io::ErrorKind::NotFound => {
                let response = BaseHttpResponse::<String> {
                    status: 500,
                    success: false,
                    data: "Username Or Password is wrong".to_string(),
                };
                return HttpResponse::InternalServerError().json(response);
            }
            _ => {
                let response = BaseHttpResponse::<String> {
                    status: 500,
                    success: false,
                    data: "uncaught error".to_string(),
                };
                HttpResponse::InternalServerError().json(response)
            }
        },
    }
}
