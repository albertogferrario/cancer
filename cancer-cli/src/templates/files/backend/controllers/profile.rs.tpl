//! Profile controller

use cancer::{
    redirect, serde_json, Auth, Inertia, InertiaProps, Request, Response,
    SavedInertiaContext, Validate,
};
use serde::Deserialize;

use crate::models::user::User;

// ============================================================================
// Show Profile
// ============================================================================

#[derive(InertiaProps)]
pub struct ProfileProps {
    pub errors: Option<serde_json::Value>,
}

pub async fn show(req: Request) -> Response {
    Inertia::render(&req, "Profile", ProfileProps { errors: None })
}

// ============================================================================
// Update Profile
// ============================================================================

#[derive(Deserialize, Validate)]
pub struct UpdateProfileRequest {
    #[validate(length(min = 2, message = "Name must be at least 2 characters"))]
    pub name: String,
    #[validate(email(message = "Please enter a valid email address"))]
    pub email: String,
}

pub async fn update(req: Request) -> Response {
    let ctx = SavedInertiaContext::from(&req);
    let user_id = Auth::user_id().ok_or_else(|| cancer::FrameworkError::AuthenticationRequired)?;
    let form: UpdateProfileRequest = req.input().await?;

    // Validate the form
    if let Err(errors) = form.validate() {
        return Inertia::render_ctx(
            &ctx,
            "Profile",
            ProfileProps {
                errors: Some(serde_json::json!(errors)),
            },
        )
        .map(|r| r.status(422));
    }

    // Get current user
    let user = User::find_by_id(user_id).await?.ok_or_else(|| {
        cancer::FrameworkError::NotFound("User not found".to_string())
    })?;

    // Check if email is already taken by another user
    if let Some(existing) = User::find_by_email(&form.email).await? {
        if existing.id != user_id {
            return Inertia::render_ctx(
                &ctx,
                "Profile",
                ProfileProps {
                    errors: Some(serde_json::json!({
                        "email": ["This email is already taken."]
                    })),
                },
            )
            .map(|r| r.status(422));
        }
    }

    // Update user
    user.update_profile(&form.name, &form.email).await?;

    redirect!("/profile").into()
}

// ============================================================================
// Update Password
// ============================================================================

#[derive(Deserialize, Validate)]
pub struct UpdatePasswordRequest {
    #[validate(length(min = 1, message = "Current password is required"))]
    pub current_password: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
    pub password_confirmation: String,
}

pub async fn update_password(req: Request) -> Response {
    let ctx = SavedInertiaContext::from(&req);
    let user_id = Auth::user_id().ok_or_else(|| cancer::FrameworkError::AuthenticationRequired)?;
    let form: UpdatePasswordRequest = req.input().await?;

    // Validate the form
    if let Err(errors) = form.validate() {
        return Inertia::render_ctx(
            &ctx,
            "Profile",
            ProfileProps {
                errors: Some(serde_json::json!(errors)),
            },
        )
        .map(|r| r.status(422));
    }

    // Check password confirmation
    if form.password != form.password_confirmation {
        return Inertia::render_ctx(
            &ctx,
            "Profile",
            ProfileProps {
                errors: Some(serde_json::json!({
                    "password_confirmation": ["Passwords do not match."]
                })),
            },
        )
        .map(|r| r.status(422));
    }

    // Get current user
    let user = User::find_by_id(user_id).await?.ok_or_else(|| {
        cancer::FrameworkError::NotFound("User not found".to_string())
    })?;

    // Verify current password
    if !user.verify_password(&form.current_password)? {
        return Inertia::render_ctx(
            &ctx,
            "Profile",
            ProfileProps {
                errors: Some(serde_json::json!({
                    "current_password": ["Current password is incorrect."]
                })),
            },
        )
        .map(|r| r.status(422));
    }

    // Update password
    user.update_password(&form.password).await?;

    redirect!("/profile").into()
}

// ============================================================================
// Delete Account
// ============================================================================

pub async fn destroy(_req: Request) -> Response {
    let user_id = Auth::user_id().ok_or_else(|| cancer::FrameworkError::AuthenticationRequired)?;

    // Get current user
    let user = User::find_by_id(user_id).await?.ok_or_else(|| {
        cancer::FrameworkError::NotFound("User not found".to_string())
    })?;

    // Delete user
    user.delete().await?;

    // Logout
    Auth::logout();

    redirect!("/").into()
}
