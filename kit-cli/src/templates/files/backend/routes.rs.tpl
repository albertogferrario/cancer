use kit::{get, group, post, routes};

use crate::controllers;
use crate::middleware;

routes! {
    // Public routes
    get!("/", controllers::home::index),

    // Guest-only routes (redirect to dashboard if logged in)
    group!("/")
        .middleware(middleware::authenticate::guest())
        .routes([
            get!("/login", controllers::auth::show_login),
            post!("/login", controllers::auth::login),
            get!("/register", controllers::auth::show_register),
            post!("/register", controllers::auth::register),
        ]),

    // Protected routes (require authentication)
    group!("/")
        .middleware(middleware::authenticate::auth())
        .routes([
            get!("/dashboard", controllers::dashboard::index),
            post!("/logout", controllers::auth::logout),
        ]),
}
