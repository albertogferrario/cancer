use cancer::{handler, json_response, redirect, route, Response};

#[handler]
pub async fn index() -> Response {
    json_response!({
        "users": [
            {"id": 1, "name": "John"},
            {"id": 2, "name": "Jane"}
        ]
    })
}

#[handler]
pub async fn show(id: i32) -> Response {
    json_response!({
        "id": id,
        "name": format!("User {id}")
    })
}

/// Example: Create a user and redirect to the user list
#[handler]
pub async fn store() -> Response {
    // ... create user logic would go here ...

    // Redirect to users.index (compile-time validated!)
    redirect!("users.index").into()
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
