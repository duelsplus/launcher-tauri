//! API client for authentication and user data operations.
//!
//! This module handles HTTP requests to the authentication API,
//! including token verification and user data retrieval.

use crate::auth::error::AuthError;
use crate::auth::models::{GetUserResponse, User, VerifyTokenResponse};

/// Base URL for the authentication API
const API_BASE_URL: &str = "https://api.venxm.uk";

/// Maximum number of retry attempts for network requests
const MAX_RETRIES: u32 = 3;

/// Initial delay in milliseconds before first retry
const INITIAL_RETRY_DELAY_MS: u64 = 100;

/// Verifies an authentication token with the API.
///
/// Sends a request to the API to verify the token and check if the user
/// is banned. The token is sent in the Authorization header without the "Bearer" prefix.
///
/// # Arguments
///
/// * `token` - The authentication token to verify
///
/// # Returns
///
/// Returns a `VerifyTokenResponse` with:
/// - `success: true` and user information if the token is valid and user is not banned
/// - `success: false` with code "banned" if the user is banned
/// - `success: false` with HTTP status code for other errors (401, 500, etc.)
/// - `success: false` with code "network_error" if the request failed
///
/// # Errors
///
/// Returns `AuthError` if there was an error parsing the response JSON.
pub async fn verify_token(token: &str) -> Result<VerifyTokenResponse, AuthError> {
    verify_token_with_base_url(token, API_BASE_URL).await
}

/// Internal function to verify token with a configurable base URL (for testing).
async fn verify_token_with_base_url(
    token: &str,
    base_url: &str,
) -> Result<VerifyTokenResponse, AuthError> {
    let client = reqwest::Client::new();
    let url = format!("{}/user", base_url);

    // Retry logic for network errors and server errors
    let mut last_error = None;
    for attempt in 0..=MAX_RETRIES {
        let response = match client.get(&url).header("Authorization", token).send().await {
            Ok(res) => res,
            Err(e) => {
                last_error = Some(e.to_string());
                // If this is the last attempt, return network error
                if attempt == MAX_RETRIES {
                    return Ok(VerifyTokenResponse {
                        success: false,
                        code: Some(crate::auth::models::VerifyCode::String(
                            "network_error".to_string(),
                        )),
                        user_id: None,
                        username: None,
                        message: last_error,
                        raw: None,
                    });
                }
                // Wait before retrying with exponential backoff
                tokio::time::sleep(tokio::time::Duration::from_millis(
                    INITIAL_RETRY_DELAY_MS * (1 << attempt),
                ))
                .await;
                continue;
            }
        };

        let status = response.status();

        // Retry on server errors (500+), but not on client errors (4xx)
        if status.as_u16() >= 500 && attempt < MAX_RETRIES {
            // Wait before retrying with exponential backoff
            tokio::time::sleep(tokio::time::Duration::from_millis(
                INITIAL_RETRY_DELAY_MS * (1 << attempt),
            ))
            .await;
            continue;
        }

        // Process the response
        return process_verify_token_response(response, status).await;
    }

    // Should never reach here, but handle it anyway
    Ok(VerifyTokenResponse {
        success: false,
        code: Some(crate::auth::models::VerifyCode::String(
            "network_error".to_string(),
        )),
        user_id: None,
        username: None,
        message: last_error,
        raw: None,
    })
}

/// Processes the response from verify_token request.
async fn process_verify_token_response(
    response: reqwest::Response,
    status: reqwest::StatusCode,
) -> Result<VerifyTokenResponse, AuthError> {
    // Handle successful response (200 OK)
    if status == 200 {
        let user: User = response.json().await?;

        // Serialize user to JSON before extracting fields to avoid borrow checker issues
        let user_json = serde_json::to_value(&user)?;
        let user_id = user.id.clone();
        let username = user.username.clone();

        // Check if user is banned
        if user.is_banned == Some(true) {
            return Ok(VerifyTokenResponse {
                success: false,
                code: Some(crate::auth::models::VerifyCode::String(
                    "banned".to_string(),
                )),
                user_id: None,
                username: None,
                message: None,
                raw: Some(user_json),
            });
        }

        // Token is valid and user is not banned
        return Ok(VerifyTokenResponse {
            success: true,
            code: None,
            user_id: Some(user_id),
            username: Some(username),
            message: None,
            raw: Some(user_json),
        });
    }

    // Handle unauthorized (401)
    if status == 401 {
        return Ok(VerifyTokenResponse {
            success: false,
            code: Some(crate::auth::models::VerifyCode::Number(401)),
            user_id: None,
            username: None,
            message: None,
            raw: None,
        });
    }

    // Handle server errors (500+)
    if status.as_u16() >= 500 {
        return Ok(VerifyTokenResponse {
            success: false,
            code: Some(crate::auth::models::VerifyCode::Number(500)),
            user_id: None,
            username: None,
            message: None,
            raw: None,
        });
    }

    // Handle other HTTP status codes
    Ok(VerifyTokenResponse {
        success: false,
        code: Some(crate::auth::models::VerifyCode::Number(status.as_u16())),
        user_id: None,
        username: None,
        message: None,
        raw: None,
    })
}

/// Retrieves user data from the API.
///
/// Sends a request to get the full user object from the API.
/// The token is sent in the Authorization header with the "Bearer" prefix.
///
/// # Arguments
///
/// * `token` - The authentication token
///
/// # Returns
///
/// Returns a `GetUserResponse` with:
/// - `success: true` and user data if the request was successful
/// - `success: false` with HTTP status code for errors (401, 500, etc.)
///
/// # Errors
///
/// Returns `AuthError` if there was a network error or error parsing the response JSON.
pub async fn get_user(token: &str) -> Result<GetUserResponse, AuthError> {
    get_user_with_base_url(token, API_BASE_URL).await
}

/// Internal function to get user with a configurable base URL (for testing).
async fn get_user_with_base_url(token: &str, base_url: &str) -> Result<GetUserResponse, AuthError> {
    let client = reqwest::Client::new();
    let url = format!("{}/user", base_url);
    let auth_header = format!("Bearer {}", token);

    // Retry logic for network errors and server errors
    for attempt in 0..=MAX_RETRIES {
        let response = match client
            .get(&url)
            .header("Authorization", &auth_header)
            .send()
            .await
        {
            Ok(res) => res,
            Err(e) => {
                // If this is the last attempt, return network error
                if attempt == MAX_RETRIES {
                    return Err(AuthError::Network(e));
                }
                // Wait before retrying with exponential backoff
                tokio::time::sleep(tokio::time::Duration::from_millis(
                    INITIAL_RETRY_DELAY_MS * (1 << attempt),
                ))
                .await;
                continue;
            }
        };

        let status = response.status();

        // Retry on server errors (500+), but not on client errors (4xx)
        if status.as_u16() >= 500 && attempt < MAX_RETRIES {
            // Wait before retrying with exponential backoff
            tokio::time::sleep(tokio::time::Duration::from_millis(
                INITIAL_RETRY_DELAY_MS * (1 << attempt),
            ))
            .await;
            continue;
        }

        // Process the response
        return process_get_user_response(response, status).await;
    }

    // Should never reach here, but return a generic error
    Err(AuthError::Unknown("Max retries exceeded".to_string()))
}

/// Processes the response from get_user request.
async fn process_get_user_response(
    response: reqwest::Response,
    status: reqwest::StatusCode,
) -> Result<GetUserResponse, AuthError> {
    // Handle successful response (200 OK)
    if status == 200 {
        let data: serde_json::Value = response.json().await?;
        return Ok(GetUserResponse {
            success: true,
            code: None,
            data: Some(data),
            message: None,
        });
    }

    // Handle unauthorized (401)
    if status == 401 {
        return Ok(GetUserResponse {
            success: false,
            code: Some(crate::auth::models::GetUserCode::Number(401)),
            data: None,
            message: None,
        });
    }

    // Handle server errors (500+)
    if status.as_u16() >= 500 {
        return Ok(GetUserResponse {
            success: false,
            code: Some(crate::auth::models::GetUserCode::Number(500)),
            data: None,
            message: None,
        });
    }

    // Handle other HTTP status codes
    Ok(GetUserResponse {
        success: false,
        code: Some(crate::auth::models::GetUserCode::Number(status.as_u16())),
        data: None,
        message: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::token;
    use mockito::{Matcher, Server};

    #[tokio::test]
    async fn test_verify_token_success() {
        let mut server = Server::new_async().await;
        let mock_server = server
            .mock("GET", "/user")
            .match_header("Authorization", Matcher::Exact("test_token".to_string()))
            .with_status(200)
            .with_body(
                r#"{
                "id": "user123",
                "username": "testuser",
                "isBanned": false
            }"#,
            )
            .create();

        let result = verify_token_with_base_url("test_token", &server.url())
            .await
            .unwrap();

        assert!(result.success);
        assert_eq!(result.user_id, Some("user123".to_string()));
        assert_eq!(result.username, Some("testuser".to_string()));
        assert!(result.code.is_none());
        assert!(result.raw.is_some());

        mock_server.assert();
    }

    #[tokio::test]
    async fn test_verify_token_banned_user() {
        let mut server = Server::new_async().await;
        let mock_server = server
            .mock("GET", "/user")
            .match_header("Authorization", Matcher::Exact("banned_token".to_string()))
            .with_status(200)
            .with_body(
                r#"{
                "id": "user123",
                "username": "banneduser",
                "isBanned": true
            }"#,
            )
            .create();

        let result = verify_token_with_base_url("banned_token", &server.url())
            .await
            .unwrap();

        assert!(!result.success);
        match result.code {
            Some(crate::auth::models::VerifyCode::String(code)) => {
                assert_eq!(code, "banned");
            }
            _ => panic!("Expected banned code"),
        }
        assert!(result.user_id.is_none());
        assert!(result.username.is_none());
        assert!(result.raw.is_some());

        mock_server.assert();
    }

    #[tokio::test]
    async fn test_verify_token_unauthorized() {
        let mut server = Server::new_async().await;
        let mock_server = server
            .mock("GET", "/user")
            .match_header("Authorization", Matcher::Exact("invalid_token".to_string()))
            .with_status(401)
            .create();

        let result = verify_token_with_base_url("invalid_token", &server.url())
            .await
            .unwrap();

        assert!(!result.success);
        match result.code {
            Some(crate::auth::models::VerifyCode::Number(401)) => {}
            _ => panic!("Expected 401 code"),
        }
        assert!(result.user_id.is_none());
        assert!(result.username.is_none());
        assert!(result.raw.is_none());

        mock_server.assert();
    }

    #[tokio::test]
    async fn test_verify_token_server_error() {
        let mut server = Server::new_async().await;
        // With retry logic, we'll retry MAX_RETRIES + 1 times (4 total attempts)
        let mocks: Vec<_> = (0..=MAX_RETRIES)
            .map(|_| {
                server
                    .mock("GET", "/user")
                    .match_header("Authorization", Matcher::Exact("token".to_string()))
                    .with_status(500)
                    .create()
            })
            .collect();

        let result = verify_token_with_base_url("token", &server.url())
            .await
            .unwrap();

        assert!(!result.success);
        match result.code {
            Some(crate::auth::models::VerifyCode::Number(500)) => {}
            _ => panic!("Expected 500 code"),
        }

        // Verify all retries were attempted
        for mock in mocks {
            mock.assert();
        }
    }

    #[tokio::test]
    async fn test_verify_token_network_error() {
        // Use an invalid URL to simulate network error
        let result = verify_token_with_base_url("token", "http://127.0.0.1:0")
            .await
            .unwrap();

        assert!(!result.success);
        match result.code {
            Some(crate::auth::models::VerifyCode::String(code)) => {
                assert_eq!(code, "network_error");
            }
            _ => panic!("Expected network_error code"),
        }
        assert!(result.message.is_some());
    }

    #[tokio::test]
    async fn test_get_user_success() {
        let mut server = Server::new_async().await;
        let user_data = serde_json::json!({
            "id": "user123",
            "username": "testuser",
            "email": "test@example.com"
        });

        let mock_server = server
            .mock("GET", "/user")
            .match_header(
                "Authorization",
                Matcher::Exact("Bearer test_token".to_string()),
            )
            .with_status(200)
            .with_body(serde_json::to_string(&user_data).unwrap())
            .create();

        let result = get_user_with_base_url("test_token", &server.url())
            .await
            .unwrap();

        assert!(result.success);
        assert!(result.data.is_some());
        assert!(result.code.is_none());
        assert!(result.message.is_none());

        let data = result.data.unwrap();
        assert_eq!(data["id"], "user123");
        assert_eq!(data["username"], "testuser");

        mock_server.assert();
    }

    #[tokio::test]
    async fn test_get_user_unauthorized() {
        let mut server = Server::new_async().await;
        let mock_server = server
            .mock("GET", "/user")
            .match_header(
                "Authorization",
                Matcher::Exact("Bearer invalid_token".to_string()),
            )
            .with_status(401)
            .create();

        let result = get_user_with_base_url("invalid_token", &server.url())
            .await
            .unwrap();

        assert!(!result.success);
        match result.code {
            Some(crate::auth::models::GetUserCode::Number(401)) => {}
            _ => panic!("Expected 401 code"),
        }
        assert!(result.data.is_none());

        mock_server.assert();
    }

    #[tokio::test]
    async fn test_get_user_server_error() {
        let mut server = Server::new_async().await;
        // With retry logic, we'll retry MAX_RETRIES + 1 times (4 total attempts)
        let mocks: Vec<_> = (0..=MAX_RETRIES)
            .map(|_| {
                server
                    .mock("GET", "/user")
                    .match_header("Authorization", Matcher::Exact("Bearer token".to_string()))
                    .with_status(500)
                    .create()
            })
            .collect();

        let result = get_user_with_base_url("token", &server.url())
            .await
            .unwrap();

        assert!(!result.success);
        match result.code {
            Some(crate::auth::models::GetUserCode::Number(500)) => {}
            _ => panic!("Expected 500 code"),
        }
        assert!(result.data.is_none());

        // Verify all retries were attempted
        for mock in mocks {
            mock.assert();
        }
    }

    #[tokio::test]
    async fn test_get_user_network_error() {
        // Use an invalid URL to simulate network error
        // This should return an AuthError::Network
        let result = get_user_with_base_url("token", "http://127.0.0.1:0").await;
        assert!(result.is_err());
        match result.unwrap_err() {
            AuthError::Network(_) => {}
            _ => panic!("Expected Network error"),
        }
    }

    #[tokio::test]
    async fn test_integration_save_load_and_verify_token() {
        // Clean up any existing token file
        let _ = token::delete_token().await;

        let test_token = "integration_test_token_12345".to_string();

        // Step 1: Save token to actual location
        token::save_token(test_token.clone()).await.unwrap();

        // Step 2: Load token from actual location
        let loaded_token = token::get_token().await.unwrap();
        assert_eq!(loaded_token, Some(test_token.clone()));

        // Step 3: Use loaded token to authenticate with mocked API
        let mut server = Server::new_async().await;
        let mock_server = server
            .mock("GET", "/user")
            .match_header("Authorization", Matcher::Exact(test_token.clone()))
            .with_status(200)
            .with_body(
                r#"{
                "id": "integration_user123",
                "username": "integration_user",
                "isBanned": false
            }"#,
            )
            .create();

        let result = verify_token_with_base_url(&test_token, &server.url())
            .await
            .unwrap();

        // Step 4: Verify authentication succeeded
        assert!(result.success);
        assert_eq!(result.user_id, Some("integration_user123".to_string()));
        assert_eq!(result.username, Some("integration_user".to_string()));
        assert!(result.code.is_none());

        mock_server.assert();

        // Clean up
        let _ = token::delete_token().await;
    }

    #[tokio::test]
    async fn test_integration_save_load_and_get_user() {
        // Clean up any existing token file
        let _ = token::delete_token().await;

        let test_token = "integration_get_user_token".to_string();

        // Step 1: Save token to actual location
        token::save_token(test_token.clone()).await.unwrap();

        // Step 2: Load token from actual location
        let loaded_token = token::get_token().await.unwrap().unwrap();

        // Step 3: Use loaded token to get user data from mocked API
        let mut server = Server::new_async().await;
        let user_data = serde_json::json!({
            "id": "integration_user456",
            "username": "integration_user2",
            "email": "integration@example.com"
        });

        let mock_server = server
            .mock("GET", "/user")
            .match_header(
                "Authorization",
                Matcher::Exact(format!("Bearer {}", loaded_token)),
            )
            .with_status(200)
            .with_body(serde_json::to_string(&user_data).unwrap())
            .create();

        let result = get_user_with_base_url(&loaded_token, &server.url())
            .await
            .unwrap();

        // Step 4: Verify user data retrieval succeeded
        assert!(result.success);
        assert!(result.data.is_some());
        let data = result.data.unwrap();
        assert_eq!(data["id"], "integration_user456");
        assert_eq!(data["username"], "integration_user2");

        mock_server.assert();

        // Clean up
        let _ = token::delete_token().await;
    }

    #[tokio::test]
    async fn test_retry_on_server_error_then_success() {
        let mut server = Server::new_async().await;
        let test_token = "retry_server_error_token".to_string();

        // First two requests return 500, third succeeds
        let mock_fail1 = server
            .mock("GET", "/user")
            .match_header("Authorization", Matcher::Exact(test_token.clone()))
            .with_status(500)
            .create();

        let mock_fail2 = server
            .mock("GET", "/user")
            .match_header("Authorization", Matcher::Exact(test_token.clone()))
            .with_status(500)
            .create();

        let mock_success = server
            .mock("GET", "/user")
            .match_header("Authorization", Matcher::Exact(test_token.clone()))
            .with_status(200)
            .with_body(
                r#"{
                "id": "retry_user123",
                "username": "retry_user",
                "isBanned": false
            }"#,
            )
            .create();

        let result = verify_token_with_base_url(&test_token, &server.url())
            .await
            .unwrap();

        // Should eventually succeed after retries
        assert!(result.success);
        assert_eq!(result.user_id, Some("retry_user123".to_string()));
        assert_eq!(result.username, Some("retry_user".to_string()));

        mock_fail1.assert();
        mock_fail2.assert();
        mock_success.assert();
    }

    #[tokio::test]
    async fn test_retry_exhausts_on_server_error() {
        let mut server = Server::new_async().await;
        let test_token = "retry_exhaust_token".to_string();

        // All requests return 500 (MAX_RETRIES + 1 attempts = 4 total)
        let mocks: Vec<_> = (0..=MAX_RETRIES)
            .map(|_| {
                server
                    .mock("GET", "/user")
                    .match_header("Authorization", Matcher::Exact(test_token.clone()))
                    .with_status(500)
                    .create()
            })
            .collect();

        let result = verify_token_with_base_url(&test_token, &server.url())
            .await
            .unwrap();

        // Should fail after all retries exhausted
        assert!(!result.success);
        match result.code {
            Some(crate::auth::models::VerifyCode::Number(500)) => {}
            _ => panic!("Expected 500 code after retries exhausted"),
        }

        // Verify all retries were attempted
        for mock in mocks {
            mock.assert();
        }
    }

    #[tokio::test]
    async fn test_no_retry_on_client_error() {
        let mut server = Server::new_async().await;
        let test_token = "no_retry_token".to_string();

        // 401 should not be retried
        let mock_unauthorized = server
            .mock("GET", "/user")
            .match_header("Authorization", Matcher::Exact(test_token.clone()))
            .with_status(401)
            .create();

        let result = verify_token_with_base_url(&test_token, &server.url())
            .await
            .unwrap();

        // Should fail immediately without retries
        assert!(!result.success);
        match result.code {
            Some(crate::auth::models::VerifyCode::Number(401)) => {}
            _ => panic!("Expected 401 code"),
        }

        // Should only be called once (no retries)
        mock_unauthorized.assert();
    }

    #[tokio::test]
    async fn test_get_user_retry_on_server_error_then_success() {
        let mut server = Server::new_async().await;
        let test_token = "get_user_retry_token".to_string();

        // First request returns 500, second succeeds
        let mock_fail = server
            .mock("GET", "/user")
            .match_header(
                "Authorization",
                Matcher::Exact(format!("Bearer {}", test_token)),
            )
            .with_status(500)
            .create();

        let user_data = serde_json::json!({
            "id": "retry_user456",
            "username": "retry_user2",
            "email": "retry@example.com"
        });

        let mock_success = server
            .mock("GET", "/user")
            .match_header(
                "Authorization",
                Matcher::Exact(format!("Bearer {}", test_token)),
            )
            .with_status(200)
            .with_body(serde_json::to_string(&user_data).unwrap())
            .create();

        let result = get_user_with_base_url(&test_token, &server.url())
            .await
            .unwrap();

        // Should eventually succeed after retry
        assert!(result.success);
        assert!(result.data.is_some());
        let data = result.data.unwrap();
        assert_eq!(data["id"], "retry_user456");

        mock_fail.assert();
        mock_success.assert();
    }

    #[tokio::test]
    async fn test_get_user_no_retry_on_client_error() {
        let mut server = Server::new_async().await;
        let test_token = "get_user_no_retry_token".to_string();

        // 403 should not be retried
        let mock_forbidden = server
            .mock("GET", "/user")
            .match_header(
                "Authorization",
                Matcher::Exact(format!("Bearer {}", test_token)),
            )
            .with_status(403)
            .create();

        let result = get_user_with_base_url(&test_token, &server.url())
            .await
            .unwrap();

        // Should fail immediately without retries
        assert!(!result.success);
        match result.code {
            Some(crate::auth::models::GetUserCode::Number(403)) => {}
            _ => panic!("Expected 403 code"),
        }

        // Should only be called once (no retries)
        mock_forbidden.assert();
    }
}
