use ferro_rs::{handler, json_response, redirect, route, HttpResponse, Response};

/// GET /users - List all users
#[handler]
pub async fn index() -> Response {
    json_response!({
        "users": [
            {"id": 1, "name": "John"},
            {"id": 2, "name": "Jane"}
        ]
    })
}

/// GET /users/create - Show form to create a new user
#[handler]
pub async fn create() -> Response {
    json_response!({
        "form": "create_user"
    })
}

/// POST /users - Store a new user
#[handler]
pub async fn store() -> Response {
    // ... create user logic would go here ...

    // Redirect to users.index (compile-time validated!)
    redirect!("users.index").into()
}

/// GET /users/{id} - Show a single user
#[handler]
pub async fn show(id: i32) -> Response {
    json_response!({
        "id": id,
        "name": format!("User {id}")
    })
}

/// GET /users/{id}/edit - Show form to edit a user
#[handler]
pub async fn edit(id: i32) -> Response {
    json_response!({
        "form": "edit_user",
        "id": id
    })
}

/// PUT /users/{id} - Update a user
#[handler]
pub async fn update(id: i32) -> Response {
    json_response!({
        "updated": id
    })
}

/// DELETE /users/{id} - Delete a user
#[handler]
pub async fn destroy(_id: i32) -> Response {
    Ok(HttpResponse::new().status(204))
}

/// Example: Redirect to a specific user with query params
#[handler]
pub async fn redirect_example() -> Response {
    // Generate a URL using route()
    let url = route("users.show", &[("id", "42")]);
    println!("Generated URL: {:?}", url);

    // Redirect with query parameters (compile-time validated!)
    redirect!("users.index")
        .query("page", "1")
        .query("sort", "name")
        .into()
}
