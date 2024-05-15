use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use bcrypt::{hash, DEFAULT_COST};
use sqlx;

use super::models::{User, RegisterUser, UpdateUser};
use crate::AppState;

#[get("/users")]
async fn get_users(app_state: web::Data<AppState>) -> impl Responder {
    let result = sqlx::query!("SELECT * FROM users")
        .fetch_all(&app_state.postgres_client)
        .await;

    match result {
        Ok(users) => HttpResponse::Ok().json(
            users
                .iter()
                .map(|user| User {
                    id: user.id,
                    name: user.name.clone(),
                    email: user.email.clone(),
                    password: user.password.clone(),
                })
                .collect::<Vec<User>>(),
        ),
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch users"),
    }
}

#[post("/users")]
async fn create_user(
    app_state: web::Data<AppState>,
    user: web::Json<RegisterUser>,
) -> impl Responder {
    let hashed_password = hash(&user.password, DEFAULT_COST).expect("Failed on hash password");
    if hashed_password == user.password {
        return HttpResponse::InternalServerError().body("Failed to hash password");
    }

    let result = sqlx::query!(
        "INSERT INTO users (name, email, password) VALUES ($1, $2, $3) RETURNING id, name, email, password",
        user.name,
        user.email,
        hashed_password
    )
    .fetch_one(&app_state.postgres_client)
    .await;

    match result {
        Ok(user) => HttpResponse::Created().json(User {
            id: user.id,
            name: user.name.clone(),
            email: user.email.clone(),
            password: user.password.clone(),
        
        }),
        Err(_) => HttpResponse::InternalServerError().body("Failed to create user"),
    }
}

#[put("/users/{id}")]
async fn update_user(
    app_state: web::Data<AppState>,
    user: web::Json<UpdateUser>,
    id: web::Path<i32>,
) -> impl Responder {
    let hashed_password = hash(&user.password, DEFAULT_COST).expect("Failed on hash password");
    if hashed_password == user.password {
        return HttpResponse::InternalServerError().body("Failed to hash password");
    }

    let result = sqlx::query!(
        "UPDATE users SET name = $1, email = $2, password = $3 WHERE id = $4 RETURNING id, name, email, password",
        user.name,
        user.email,
        hashed_password,
        id.into_inner()
    )
    .fetch_one(&app_state.postgres_client)
    .await;

    match result {
        Ok(user) => HttpResponse::Ok().json(User {
            id: user.id,
            name: user.name.clone(),
            email: user.email.clone(),
            password: user.password.clone(),
        }),
        Err(_) => HttpResponse::InternalServerError().body("Failed to update user"),
    }
}

#[delete("/users/{id}")]
async fn delete_user(
    app_state: web::Data<AppState>,
    id: web::Path<i32>,
) -> impl Responder {
    let result = sqlx::query!(
        "DELETE FROM users WHERE id = $1 RETURNING id, name, email, password",
        id.into_inner()
    )
    .fetch_one(&app_state.postgres_client)
    .await;

    match result {
        Ok(user) => HttpResponse::Ok().json(User {
            id: user.id,
            name: user.name.clone(),
            email: user.email.clone(),
            password: user.password.clone(),
        }),
        Err(_) => HttpResponse::InternalServerError().body("Failed to delete user"),
    }
}

pub fn users_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_users);
    cfg.service(create_user);
    cfg.service(update_user);
    cfg.service(delete_user);
}