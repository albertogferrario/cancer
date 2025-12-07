use kit::{text_response, Request, Response, ResponseExt};

pub async fn index(_req: Request) -> Response {
    text_response!("Welcome to Kit!")
        .status(200)
        .header("content-type", "text/html")
}
