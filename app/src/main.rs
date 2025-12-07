use kit::Server;

mod controllers;
mod routes;

#[tokio::main]
async fn main() {
    let router = routes::register();

    Server::new(router)
        .port(8090)
        .run()
        .await
        .expect("Failed to start server");
}
