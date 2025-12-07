use kit::{inertia_response, Request, Response};

pub async fn index(_req: Request) -> Response {
    inertia_response!("Home", {
        "title": "Welcome to Kit!",
        "message": "Your Inertia + React app is ready."
    })
}
