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

    // Get the .env variables
    let db_url = std::env::var("DATABASE_URL")
        .expect("Missing `DATABASE_URL` env variable, needed for running the server");
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    let state = State::new(&db_url).await?;

    let mut app = tide::with_state(state);

    student_router(&mut app).await;

    app.listen(format!("127.0.0.1:{}", port)).await?;
    Ok(())
}
