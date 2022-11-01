use super::state::State;
use tide::prelude::*;
use tide::Request;
use tide::Response;

use redis::JsonCommands;
use redis_derive::{FromRedisValue, ToRedisArgs};

#[derive(Debug, Clone, Deserialize, Serialize, FromRedisValue, ToRedisArgs)]
struct StudentData {
    name: String,
    email: String,
    major: String,
    level: i8,
}

#[derive(Debug, Clone, Deserialize, Serialize, FromRedisValue, ToRedisArgs)]
pub struct Student {
    code: String,
    data: StudentData,
}

pub async fn _get(_req: Request<State>) -> tide::Result<()> {
    Ok(())
}

pub async fn list(req: Request<State>) -> tide::Result<Response> {
    let mut conn = req
        .state()
        .db
        .get_connection()
        .expect("Coundn't connect with redis");

    let query: Vec<Student> = conn.json_get("students", "$")?;

    let mut res = tide::Response::new(200);
    res.set_body(tide::Body::from_json(&query)?);

    Ok(res)
}

pub async fn create(req: Request<State>) -> tide::Result<Response> {
    let _conn = req
        .state()
        .db
        .get_connection()
        .expect("Coundn't connect with redis");

    Ok(tide::Response::new(200))
}
