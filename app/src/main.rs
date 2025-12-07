use kit::{Router, Server};

mod controllers;

#[tokio::main]
async fn main() {
    let router = Router::new()
        .get("/", controllers::home::index).name("home")
        .get("/users", controllers::user::index).name("users.index")
        .get("/users/{id}", controllers::user::show).name("users.show")
        .post("/users", controllers::user::store).name("users.store")
        .get("/redirect-example", controllers::user::redirect_example);

    Server::new(router)
        .port(8090)
        .run()
        .await
        .expect("Failed to start server");
}
