use kit::{json_response, Request, Response, ResponseExt};

pub async fn index(_req: Request) -> Response {
    json_response!({
        "users": [
            {"id": 1, "name": "John"},
            {"id": 2, "name": "Jane"}
        ]
    }).status(200)

}

pub async fn show(req: Request) -> Response {
    let id = req.param("id")?;
    json_response!({
        "id": id,
        "name": format!("User {}", id)
    })
}
