use kit::{get, post, routes};

use crate::controllers;
use crate::middleware::AuthMiddleware;

routes! {
    get("/", controllers::home::index).name("home"),
    get("/users", controllers::user::index).name("users.index"),
    get("/users/{id}", controllers::user::show).name("users.show"),
    post("/users", controllers::user::store).name("users.store"),
    get("/redirect-example", controllers::user::redirect_example),
    get("/config", controllers::config_example::show).name("config.show"),
    // Protected route - requires Authorization header
    get("/protected", controllers::home::index).middleware(AuthMiddleware),
}
