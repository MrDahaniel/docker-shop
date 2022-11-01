use super::State;
use crate::students;

use tide::Server;

pub async fn student_router(app: &mut Server<State>) {
    app.at("/").get(students::list);

    app.at("/create").post(students::create);
}
