extern crate dotenv;

mod router;
mod state;
mod students;

use async_std::main;
use router::student_router;
use state::State;

#[main]
async fn main() -> tide::Result<()> {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt::init();

    // Get the .env variables
    let db_url = std::env::var("DATABASE_URL").unwrap(); //_or_else(|_| "redis://redis_db:6379".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    let state = State::new(&db_url).await?;

    let mut app = tide::with_state(state);

    student_router(&mut app).await;

    app.listen(format!("0.0.0.0:{}", port)).await?;
    Ok(())
}
