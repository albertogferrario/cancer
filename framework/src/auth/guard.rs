//! Authentication guard (facade)

use crate::session::{
    auth_user_id, clear_auth_user, generate_csrf_token, regenerate_session_id, session_mut,
    set_auth_user,
};

/// Authentication facade
///
/// Provides Laravel-like static methods for authentication operations.
///
/// # Example
///
/// ```rust,ignore
/// use kit::Auth;
///
/// // Check if authenticated
/// if Auth::check() {
///     let user_id = Auth::id().unwrap();
/// }
///
/// // Log in
/// Auth::login(user_id);
///
/// // Log out
/// Auth::logout();
/// ```
pub struct Auth;

impl Auth {
    /// Get the authenticated user's ID
    ///
    /// Returns None if not authenticated.
    pub fn id() -> Option<i64> {
        auth_user_id()
    }

    /// Check if a user is currently authenticated
    pub fn check() -> bool {
        Self::id().is_some()
    }

    /// Check if the current user is a guest (not authenticated)
    pub fn guest() -> bool {
        !Self::check()
    }

    /// Log in a user by their ID
    ///
    /// This sets the user ID in the session, making them authenticated.
    ///
    /// # Security
    ///
    /// This method regenerates the session ID to prevent session fixation attacks.
    pub fn login(user_id: i64) {
        // Regenerate session ID to prevent session fixation
        regenerate_session_id();

        // Set the authenticated user
        set_auth_user(user_id);

        // Regenerate CSRF token for extra security
        session_mut(|session| {
            session.csrf_token = generate_csrf_token();
        });
    }

    /// Log in a user with "remember me" functionality
    ///
    /// This extends the session lifetime for persistent login.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user's ID
    /// * `remember_token` - A secure token for remember me cookie
    pub fn login_remember(user_id: i64, _remember_token: &str) {
        // For now, just do a regular login
        // Remember me cookie handling is done in the controller
        Self::login(user_id);
    }

    /// Log out the current user
    ///
    /// Clears the authenticated user from the session.
    ///
    /// # Security
    ///
    /// This regenerates the CSRF token to prevent any cached tokens from being reused.
    pub fn logout() {
        // Clear the authenticated user
        clear_auth_user();

        // Regenerate CSRF token for security
        session_mut(|session| {
            session.csrf_token = generate_csrf_token();
        });
    }

    /// Log out and invalidate the entire session
    ///
    /// Use this for complete session destruction (e.g., "logout everywhere").
    pub fn logout_and_invalidate() {
        session_mut(|session| {
            session.flush();
            session.csrf_token = generate_csrf_token();
        });
    }

    /// Attempt to authenticate with a validator function
    ///
    /// The validator function should return the user ID if credentials are valid.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let user_id = Auth::attempt(async {
    ///     // Validate credentials
    ///     let user = User::find_by_email(&email).await?;
    ///     if user.verify_password(&password)? {
    ///         Ok(Some(user.id))
    ///     } else {
    ///         Ok(None)
    ///     }
    /// }).await?;
    ///
    /// if let Some(id) = user_id {
    ///     // Authentication successful
    /// }
    /// ```
    pub async fn attempt<F, Fut>(validator: F) -> Result<Option<i64>, crate::error::FrameworkError>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<Option<i64>, crate::error::FrameworkError>>,
    {
        let result = validator().await?;
        if let Some(user_id) = result {
            Self::login(user_id);
        }
        Ok(result)
    }

    /// Validate credentials without logging in
    ///
    /// Useful for password confirmation dialogs.
    pub async fn validate<F, Fut>(validator: F) -> Result<bool, crate::error::FrameworkError>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<bool, crate::error::FrameworkError>>,
    {
        validator().await
    }
}
