//! Authentication controller

use cancer::{
    redirect, serde_json, Auth, Inertia, InertiaProps, Request, Response,
    SavedInertiaContext, Validate,
};
use serde::Deserialize;

use crate::models::user::User;

// ============================================================================
// Login
// ============================================================================

#[derive(InertiaProps)]
pub struct LoginProps {
    pub errors: Option<serde_json::Value>,
}

pub async fn show_login(req: Request) -> Response {
    Inertia::render(&req, "auth/Login", LoginProps { errors: None })
}

#[derive(Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Please enter a valid email address"))]
    pub email: String,
    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
    #[serde(default)]
    pub remember: bool,
}

pub async fn login(req: Request) -> Response {
    // Save Inertia context before consuming request
    let ctx = SavedInertiaContext::from(&req);

    let form: LoginRequest = req.input().await?;

    // Validate the form
    if let Err(errors) = form.validate() {
        return Inertia::render_ctx(
            &ctx,
            "auth/Login",
            LoginProps {
                errors: Some(serde_json::json!(errors)),
            },
        )
        .map(|r| r.status(422));
    }

    // Find user by email
    let user = match User::find_by_email(&form.email).await? {
        Some(u) => u,
        None => {
            return Inertia::render_ctx(
                &ctx,
                "auth/Login",
                LoginProps {
                    errors: Some(serde_json::json!({
                        "email": ["These credentials do not match our records."]
                    })),
                },
            )
            .map(|r| r.status(422));
        }
    };

    // Verify password
    if !user.verify_password(&form.password)? {
        return Inertia::render_ctx(
            &ctx,
            "auth/Login",
            LoginProps {
                errors: Some(serde_json::json!({
                    "email": ["These credentials do not match our records."]
                })),
            },
        )
        .map(|r| r.status(422));
    }

    // Log in the user
    Auth::login(user.id);

    // Handle remember me
    if form.remember {
        // Generate and store remember token
        let token = cancer::session::generate_session_id();
        user.update_remember_token(Some(token)).await?;
    }

    redirect!("/dashboard").into()
}

// ============================================================================
// Registration
// ============================================================================

#[derive(InertiaProps)]
pub struct RegisterProps {
    pub errors: Option<serde_json::Value>,
}

pub async fn show_register(req: Request) -> Response {
    Inertia::render(&req, "auth/Register", RegisterProps { errors: None })
}

#[derive(Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 2, message = "Name must be at least 2 characters"))]
    pub name: String,
    #[validate(email(message = "Please enter a valid email address"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
    pub password_confirmation: String,
}

pub async fn register(req: Request) -> Response {
    // Save Inertia context before consuming request
    let ctx = SavedInertiaContext::from(&req);

    let form: RegisterRequest = req.input().await?;

    // Validate the form
    if let Err(errors) = form.validate() {
        return Inertia::render_ctx(
            &ctx,
            "auth/Register",
            RegisterProps {
                errors: Some(serde_json::json!(errors)),
            },
        )
        .map(|r| r.status(422));
    }

    // Check password confirmation
    if form.password != form.password_confirmation {
        return Inertia::render_ctx(
            &ctx,
            "auth/Register",
            RegisterProps {
                errors: Some(serde_json::json!({
                    "password_confirmation": ["Passwords do not match."]
                })),
            },
        )
        .map(|r| r.status(422));
    }

    // Check if email already exists
    if User::find_by_email(&form.email).await?.is_some() {
        return Inertia::render_ctx(
            &ctx,
            "auth/Register",
            RegisterProps {
                errors: Some(serde_json::json!({
                    "email": ["This email is already registered."]
                })),
            },
        )
        .map(|r| r.status(422));
    }

    // Create user
    let user = User::create(&form.name, &form.email, &form.password).await?;

    // Log in the new user
    Auth::login(user.id);

    redirect!("/dashboard").into()
}

// ============================================================================
// Logout
// ============================================================================

pub async fn logout(_req: Request) -> Response {
    Auth::logout();
    redirect!("/").into()
}
