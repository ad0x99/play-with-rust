#[macro_use]
extern crate log;

extern crate pretty_env_logger;

use std::sync::Arc;

use axum::{
    routing::{delete, get, post},
    Router,
};
use dotenvy::dotenv;
use persistance::{
    answers_dao::{AnswersDao, AnswersDaoImpl},
    questions_dao::{QuestionsDao, QuestionsDaoImpl},
};
use sqlx::postgres::PgPoolOptions;

mod handlers;
mod models;
mod persistance;

use handlers::*;
use tokio::net::TcpListener;

#[derive(Clone)]
pub struct AppState {
    pub questions_dao: Arc<dyn QuestionsDao + Send + Sync>,
    pub answers_dao: Arc<dyn AnswersDao + Send + Sync>,
}

#[tokio::main]
async fn main() {
    // Initialize pretty_env_logger and dotenv
    pretty_env_logger::init();
    dotenv().ok();

    // Create a new PgPoolOptions instance with a maximum of 5 connections.
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set."))
        .await
        .expect("Failed to create Postgres connection pool!");

    let questions_dao = Arc::new(QuestionsDaoImpl::new(pool.clone()));
    let answers_dao = Arc::new(AnswersDaoImpl::new(pool.clone()));

    let app_state = AppState {
        questions_dao,
        answers_dao,
    };

    let app = Router::new()
        .route("/api/v1/question", post(create_question))
        .route("/api/v1/questions", get(read_questions))
        .route("/api/v1/question", delete(delete_question))
        .route("/api/v1/answer", post(create_answer))
        .route("/api/v1/answers", get(read_answers))
        .route("/api/v1/answer", delete(delete_answer))
        .with_state(app_state);

    let listener = TcpListener::bind("127.0.0.1:8000")
        .await
        .expect("Failed to bind to address");

    info!("Server running on {:?}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .expect("Failed to create server from listener");
}
