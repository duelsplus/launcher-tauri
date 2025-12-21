//! API client for authentication and user data operations.
//!
//! This module handles HTTP requests to the authentication API,
//! including token verification and user data retrieval.

use crate::auth::error::AuthError;
use crate::auth::models::{
    GetGlobalStatsResponse, GetStatsResponse, GetUserResponse, User, VerifyTokenResponse,
};

/// Base URL for the authentication API
const API_BASE_URL: &str = "https://api.venxm.uk";

/// Base URL for the public stats API
const STATS_API_URL: &str = "https://duelsplus.com/api/stats";

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

/// Processes the verify token response from the API.
async fn process_verify_token_response(
    response: reqwest::Response,
    status: reqwest::StatusCode,
) -> Result<VerifyTokenResponse, AuthError> {
    // Success case (200 OK)
    if status.is_success() {
        let user: User = response.json().await?;

        // Check if user is banned
        if user.is_banned.unwrap_or(false) {
            return Ok(VerifyTokenResponse {
                success: false,
                code: Some(crate::auth::models::VerifyCode::String(
                    "banned".to_string(),
                )),
                user_id: Some(user.id.clone()),
                username: Some(user.username.clone()),
                message: None,
                raw: Some(serde_json::to_value(&user)?),
            });
        }

        // User is not banned, return success
        return Ok(VerifyTokenResponse {
            success: true,
            code: None,
            user_id: Some(user.id.clone()),
            username: Some(user.username.clone()),
            message: None,
            raw: Some(serde_json::to_value(&user)?),
        });
    }

    // Handle 401 Unauthorized
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

/// Retrieves user statistics from the API.
///
/// Sends a request to get the user's stats from the API.
/// The token is sent in the Authorization header with the "Bearer" prefix.
///
/// # Arguments
///
/// * `token` - The authentication token
///
/// # Returns
///
/// Returns a `GetStatsResponse` with:
/// - `success: true` and stats data if the request was successful
/// - `success: false` with HTTP status code for errors (401, 500, etc.)
///
/// # Errors
///
/// Returns `AuthError` if there was a network error or error parsing the response JSON.
pub async fn get_stats(token: &str) -> Result<GetStatsResponse, AuthError> {
    get_stats_with_base_url(token, API_BASE_URL).await
}

/// Internal function to get stats with a configurable base URL (for testing).
async fn get_stats_with_base_url(
    token: &str,
    base_url: &str,
) -> Result<GetStatsResponse, AuthError> {
    let client = reqwest::Client::new();
    let url = format!("{}/user/stats", base_url);
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
        return process_get_stats_response(response, status).await;
    }

    // Should never reach here, but return a generic error
    Err(AuthError::Unknown("Max retries exceeded".to_string()))
}

/// Processes the user data response from the API.
async fn process_get_user_response(
    response: reqwest::Response,
    status: reqwest::StatusCode,
) -> Result<GetUserResponse, AuthError> {
    // Success case (200 OK)
    if status.is_success() {
        let data: serde_json::Value = response.json().await?;
        return Ok(GetUserResponse {
            success: true,
            code: None,
            data: Some(data),
            message: None,
        });
    }

    // Handle 401 Unauthorized
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

/// Processes the stats response from the API.
async fn process_get_stats_response(
    response: reqwest::Response,
    status: reqwest::StatusCode,
) -> Result<GetStatsResponse, AuthError> {
    // Success case (200 OK)
    if status.is_success() {
        let data: serde_json::Value = response.json().await?;

        return Ok(GetStatsResponse {
            success: true,
            code: None,
            stats: data.get("stats").cloned(),
            message: None,
        });
    }

    // Handle 401 Unauthorized
    if status == 401 {
        return Ok(GetStatsResponse {
            success: false,
            code: Some(crate::auth::models::GetUserCode::Number(401)),
            stats: None,
            message: None,
        });
    }

    // Handle server errors (500+)
    if status.as_u16() >= 500 {
        return Ok(GetStatsResponse {
            success: false,
            code: Some(crate::auth::models::GetUserCode::Number(500)),
            stats: None,
            message: None,
        });
    }

    // Handle other HTTP status codes
    Ok(GetStatsResponse {
        success: false,
        code: Some(crate::auth::models::GetUserCode::Number(status.as_u16())),
        stats: None,
        message: None,
    })
}

/// Checks the API health status.
///
/// Sends a GET request to the /health endpoint and checks if the API is healthy.
///
/// # Returns
///
/// Returns `true` if the API responds with `{"status":"ok"}`, `false` otherwise.
pub async fn check_api_status() -> bool {
    let client = reqwest::Client::new();
    let url = format!("{}/health", API_BASE_URL);

    match client.get(&url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                // Try to parse the response as JSON
                if let Ok(json) = response.json::<serde_json::Value>().await {
                    // Check if status is "ok"
                    json.get("status")
                        .and_then(|s| s.as_str())
                        .map(|s| s == "ok")
                        .unwrap_or(false)
                } else {
                    false
                }
            } else {
                false
            }
        }
        Err(_) => false,
    }
}

/// Retrieves global statistics from the public API.
///
/// Sends a request to get global stats from the public API (no auth required).
///
/// # Returns
///
/// Returns a `GetGlobalStatsResponse` with:
/// - `success: true` and stats data if the request was successful
/// - `success: false` with HTTP status code for errors (500, etc.)
///
/// # Errors
///
/// Returns `AuthError` if there was a network error or error parsing the response JSON.
pub async fn get_global_stats() -> Result<GetGlobalStatsResponse, AuthError> {
    let client = reqwest::Client::new();

    // Retry logic for network errors and server errors
    for attempt in 0..=MAX_RETRIES {
        let response = match client.get(STATS_API_URL).send().await {
            Ok(res) => res,
            Err(e) => {
                // If this is the last attempt, return network error
                if attempt == MAX_RETRIES {
                    return Ok(GetGlobalStatsResponse {
                        success: false,
                        code: Some(crate::auth::models::GetUserCode::String(
                            "network_error".to_string(),
                        )),
                        data: None,
                        message: Some(e.to_string()),
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
        return process_get_global_stats_response(response, status).await;
    }

    // Should never reach here, but return a generic error
    Err(AuthError::Unknown("Max retries exceeded".to_string()))
}

/// Processes the global stats response from the API.
async fn process_get_global_stats_response(
    response: reqwest::Response,
    status: reqwest::StatusCode,
) -> Result<GetGlobalStatsResponse, AuthError> {
    // Success case (200 OK)
    if status.is_success() {
        let data: serde_json::Value = response.json().await?;
        return Ok(GetGlobalStatsResponse {
            success: true,
            code: None,
            data: Some(data),
            message: None,
        });
    }

    // Handle server errors (500+)
    if status.as_u16() >= 500 {
        return Ok(GetGlobalStatsResponse {
            success: false,
            code: Some(crate::auth::models::GetUserCode::Number(500)),
            data: None,
            message: None,
        });
    }

    // Handle other HTTP status codes
    Ok(GetGlobalStatsResponse {
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
        let test_token = "test_token_123";

        let mock_server = server
            .mock("GET", "/user")
            .match_header("Authorization", Matcher::Exact(test_token.to_string()))
            .with_status(200)
            .with_body(
                r#"{
                "id": "user123",
                "username": "testuser",
                "isBanned": false
            }"#,
            )
            .create();

        let result = verify_token_with_base_url(test_token, &server.url())
            .await
            .unwrap();

        assert!(result.success);
        assert_eq!(result.user_id, Some("user123".to_string()));
        assert_eq!(result.username, Some("testuser".to_string()));
        assert!(result.code.is_none());

        mock_server.assert();
    }

    #[tokio::test]
    async fn test_verify_token_banned_user() {
        let mut server = Server::new_async().await;
        let test_token = "banned_token";

        let mock_server = server
            .mock("GET", "/user")
            .match_header("Authorization", Matcher::Exact(test_token.to_string()))
            .with_status(200)
            .with_body(
                r#"{
                "id": "banned_user",
                "username": "banneduser",
                "isBanned": true
            }"#,
            )
            .create();

        let result = verify_token_with_base_url(test_token, &server.url())
            .await
            .unwrap();

        assert!(!result.success);
        match result.code {
            Some(crate::auth::models::VerifyCode::String(code)) => {
                assert_eq!(code, "banned");
            }
            _ => panic!("Expected banned code"),
        }

        mock_server.assert();
    }

    #[tokio::test]
    async fn test_verify_token_unauthorized() {
        let mut server = Server::new_async().await;
        let test_token = "invalid_token";

        let mock_server = server
            .mock("GET", "/user")
            .match_header("Authorization", Matcher::Exact(test_token.to_string()))
            .with_status(401)
            .create();

        let result = verify_token_with_base_url(test_token, &server.url())
            .await
            .unwrap();

        assert!(!result.success);
        match result.code {
            Some(crate::auth::models::VerifyCode::Number(401)) => {}
            _ => panic!("Expected 401 code"),
        }

        mock_server.assert();
    }

    #[tokio::test]
    async fn test_verify_token_server_error() {
        let mut server = Server::new_async().await;
        let test_token = "server_error_token";

        let mocks: Vec<_> = (0..=MAX_RETRIES)
            .map(|_| {
                server
                    .mock("GET", "/user")
                    .match_header("Authorization", Matcher::Exact(test_token.to_string()))
                    .with_status(500)
                    .create()
            })
            .collect();

        let result = verify_token_with_base_url(test_token, &server.url())
            .await
            .unwrap();

        assert!(!result.success);
        match result.code {
            Some(crate::auth::models::VerifyCode::Number(500)) => {}
            _ => panic!("Expected 500 code"),
        }

        for mock in mocks {
            mock.assert();
        }
    }

    #[tokio::test]
    async fn test_verify_token_network_error() {
        // Use an invalid URL to simulate network error
        let result = verify_token_with_base_url("token", "http://invalid-url-that-does-not-exist")
            .await
            .unwrap();

        assert!(!result.success);
        match result.code {
            Some(crate::auth::models::VerifyCode::String(code)) => {
                assert_eq!(code, "network_error");
            }
            _ => panic!("Expected network_error code"),
        }
    }

    #[tokio::test]
    async fn test_get_user_success() {
        let mut server = Server::new_async().await;
        let test_token = "user_token_123";

        let mock_server = server
            .mock("GET", "/user")
            .match_header(
                "Authorization",
                Matcher::Exact(format!("Bearer {}", test_token)),
            )
            .with_status(200)
            .with_body(
                r#"{
                "id": "user456",
                "username": "getuser",
                "email": "user@example.com"
            }"#,
            )
            .create();

        let result = get_user_with_base_url(test_token, &server.url())
            .await
            .unwrap();

        assert!(result.success);
        assert!(result.data.is_some());
        let data = result.data.unwrap();
        assert_eq!(data["id"], "user456");
        assert_eq!(data["username"], "getuser");

        mock_server.assert();
    }

    #[tokio::test]
    async fn test_get_user_unauthorized() {
        let mut server = Server::new_async().await;
        let test_token = "invalid_user_token";

        let mock_server = server
            .mock("GET", "/user")
            .match_header(
                "Authorization",
                Matcher::Exact(format!("Bearer {}", test_token)),
            )
            .with_status(401)
            .create();

        let result = get_user_with_base_url(test_token, &server.url())
            .await
            .unwrap();

        assert!(!result.success);
        match result.code {
            Some(crate::auth::models::GetUserCode::Number(401)) => {}
            _ => panic!("Expected 401 code"),
        }

        mock_server.assert();
    }

    #[tokio::test]
    async fn test_get_user_server_error() {
        let mut server = Server::new_async().await;
        let test_token = "server_error_user_token";

        let mocks: Vec<_> = (0..=MAX_RETRIES)
            .map(|_| {
                server
                    .mock("GET", "/user")
                    .match_header(
                        "Authorization",
                        Matcher::Exact(format!("Bearer {}", test_token)),
                    )
                    .with_status(500)
                    .create()
            })
            .collect();

        let result = get_user_with_base_url(test_token, &server.url())
            .await
            .unwrap();

        assert!(!result.success);
        match result.code {
            Some(crate::auth::models::GetUserCode::Number(500)) => {}
            _ => panic!("Expected 500 code"),
        }

        for mock in mocks {
            mock.assert();
        }
    }

    #[tokio::test]
    async fn test_get_user_network_error() {
        let result =
            get_user_with_base_url("token", "http://invalid-url-that-does-not-exist").await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AuthError::Network(_) => {}
            _ => panic!("Expected Network error"),
        }
    }

    #[tokio::test]
    async fn test_integration_save_load_and_verify_token() {
        use tempfile::TempDir;

        // Acquire test lock to prevent parallel execution
        // Handle poisoned mutex by recovering from it
        let _lock = token::TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());

        // Create isolated test environment
        let _temp_dir = TempDir::new().unwrap();
        let temp_path = _temp_dir.path().to_path_buf();
        *token::TEST_TOKEN_DIR
            .lock()
            .unwrap_or_else(|e| e.into_inner()) = Some(temp_path);

        let test_token = "integration_test_token_12345".to_string();

        token::save_token(test_token.clone()).await.unwrap();
        let loaded_token = token::get_token().await.unwrap();
        assert_eq!(loaded_token, Some(test_token.clone()));

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

        assert!(result.success);
        assert_eq!(result.user_id, Some("integration_user123".to_string()));
        assert_eq!(result.username, Some("integration_user".to_string()));
        assert!(result.code.is_none());

        mock_server.assert();

        // Clean up
        *token::TEST_TOKEN_DIR
            .lock()
            .unwrap_or_else(|e| e.into_inner()) = None;
    }

    #[tokio::test]
    async fn test_integration_save_load_and_get_user() {
        use tempfile::TempDir;

        // Acquire test lock to prevent parallel execution
        // Handle poisoned mutex by recovering from it
        let _lock = token::TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());

        // Create isolated test environment
        let _temp_dir = TempDir::new().unwrap();
        let temp_path = _temp_dir.path().to_path_buf();
        *token::TEST_TOKEN_DIR
            .lock()
            .unwrap_or_else(|e| e.into_inner()) = Some(temp_path);

        let test_token = "integration_get_user_token".to_string();
        token::save_token(test_token.clone()).await.unwrap();
        let loaded_token = token::get_token().await.unwrap().unwrap();

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
            .create_async()
            .await;

        let result = get_user_with_base_url(&loaded_token, &server.url())
            .await
            .unwrap();

        assert!(result.success);
        assert!(result.data.is_some());
        let data = result.data.unwrap();
        assert_eq!(data["id"], "integration_user456");
        assert_eq!(data["username"], "integration_user2");

        mock_server.assert_async().await;

        // Clean up
        *token::TEST_TOKEN_DIR
            .lock()
            .unwrap_or_else(|e| e.into_inner()) = None;
    }

    #[tokio::test]
    async fn test_integration_save_load_get_user_and_stats() {
        use tempfile::TempDir;

        // Acquire test lock to prevent parallel execution
        // Handle poisoned mutex by recovering from it
        let _lock = token::TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());

        // Create isolated test environment
        let _temp_dir = TempDir::new().unwrap();
        let temp_path = _temp_dir.path().to_path_buf();
        *token::TEST_TOKEN_DIR
            .lock()
            .unwrap_or_else(|e| e.into_inner()) = Some(temp_path);

        let test_token = "stats_test_token_12345".to_string();

        token::save_token(test_token.clone()).await.unwrap();
        let loaded_token = token::get_token().await.unwrap();
        assert_eq!(loaded_token, Some(test_token.clone()));
        let mut server = Server::new_async().await;

        // Mock user endpoint
        let mock_user = server
            .mock("GET", "/user")
            .match_header(
                "Authorization",
                Matcher::Exact(format!("Bearer {}", test_token)),
            )
            .with_status(200)
            .with_body(
                r#"{
                "id": "stats_user123",
                "username": "stats_user",
                "isBanned": false
            }"#,
            )
            .create();

        // Mock stats endpoint
        let mock_stats = server
            .mock("GET", "/user/stats")
            .match_header(
                "Authorization",
                Matcher::Exact(format!("Bearer {}", test_token)),
            )
            .with_status(200)
            .with_body(
                r#"{
                "stats": {
                    "wins": 100,
                    "losses": 50,
                    "kills": 500,
                    "deaths": 250
                }
            }"#,
            )
            .create();

        let user_result = get_user_with_base_url(&test_token, &server.url())
            .await
            .unwrap();

        assert!(user_result.success);
        assert!(user_result.data.is_some());
        let data = user_result.data.unwrap();
        assert_eq!(data["id"], "stats_user123");
        assert_eq!(data["username"], "stats_user");

        println!("\n=== User Data ===");
        println!("{}", serde_json::to_string_pretty(&data).unwrap());

        let stats_result = get_stats_with_base_url(&test_token, &server.url())
            .await
            .unwrap();

        assert!(stats_result.success);
        assert!(stats_result.stats.is_some());
        let stats = stats_result.stats.unwrap();
        assert_eq!(stats["wins"], 100);
        assert_eq!(stats["losses"], 50);
        assert_eq!(stats["kills"], 500);
        assert_eq!(stats["deaths"], 250);

        println!("\n=== User Stats ===");
        println!("{}", serde_json::to_string_pretty(&stats).unwrap());

        mock_user.assert();
        mock_stats.assert();

        // Clean up
        *token::TEST_TOKEN_DIR
            .lock()
            .unwrap_or_else(|e| e.into_inner()) = None;
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

        let mock_success = server
            .mock("GET", "/user")
            .match_header(
                "Authorization",
                Matcher::Exact(format!("Bearer {}", test_token)),
            )
            .with_status(200)
            .with_body(
                r#"{
                "id": "retry_user456",
                "username": "retry_getuser"
            }"#,
            )
            .create();

        let result = get_user_with_base_url(&test_token, &server.url())
            .await
            .unwrap();

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

        // 401 should not be retried
        let mock_unauthorized = server
            .mock("GET", "/user")
            .match_header(
                "Authorization",
                Matcher::Exact(format!("Bearer {}", test_token)),
            )
            .with_status(401)
            .create();

        let result = get_user_with_base_url(&test_token, &server.url())
            .await
            .unwrap();

        assert!(!result.success);
        match result.code {
            Some(crate::auth::models::GetUserCode::Number(401)) => {}
            _ => panic!("Expected 401 code"),
        }

        // Should only be called once (no retries)
        mock_unauthorized.assert();
    }

    #[tokio::test]
    #[ignore] // Ignored by default since it makes real API calls
    async fn test_real_token_load_and_fetch_user_data() {
        // This test uses the REAL token file and makes REAL API calls
        // It's marked with #[ignore] so it won't run by default
        // Run with: cargo test test_real_token_load_and_fetch_user_data -- --ignored --nocapture

        // Acquire test lock and use real path (not isolated)
        let _lock = token::TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        *token::TEST_TOKEN_DIR
            .lock()
            .unwrap_or_else(|e| e.into_inner()) = None;

        println!("\n=== Loading Real Token ===");

        // Check if token exists
        let exists = token::token_exists().await.unwrap();
        if !exists {
            println!("❌ No real token file found. Run the app and log in first.");
            return;
        }

        println!("✓ Token file found");

        // Load the real token
        let real_token = match token::get_token().await.unwrap() {
            Some(token) => {
                println!("✓ Token loaded successfully");
                println!("  Token length: {} characters", token.len());
                println!(
                    "  Token preview: {}...",
                    if token.len() > 20 {
                        &token[..20]
                    } else {
                        &token
                    }
                );
                token
            }
            None => {
                println!("❌ Token file exists but contains no token");
                return;
            }
        };

        println!("\n=== Verifying Token with Real API ===");

        match verify_token(&real_token).await {
            Ok(result) => {
                if result.success {
                    println!("✓ Token is valid!");
                    println!("  User ID: {}", result.user_id.as_ref().unwrap());
                    println!("  Username: {}", result.username.as_ref().unwrap());
                } else {
                    println!("❌ Token verification failed");
                    if let Some(code) = result.code {
                        println!("  Error code: {:?}", code);
                    }
                    return;
                }
            }
            Err(e) => {
                println!("❌ Error verifying token: {}", e);
                return;
            }
        }

        println!("\n=== Fetching User Data from Real API ===");

        match get_user(&real_token).await {
            Ok(result) => {
                if result.success {
                    println!("✓ User data fetched successfully!");
                    if let Some(data) = result.data {
                        println!("\n{}", serde_json::to_string_pretty(&data).unwrap());
                    }
                } else {
                    println!("❌ Failed to fetch user data");
                    if let Some(code) = result.code {
                        println!("  Error code: {:?}", code);
                    }
                }
            }
            Err(e) => {
                println!("❌ Error fetching user data: {}", e);
            }
        }

        println!("\n=== Fetching User Stats from Real API ===");

        match get_stats(&real_token).await {
            Ok(result) => {
                if result.success {
                    println!("✓ User stats fetched successfully!");
                    if let Some(stats) = result.stats {
                        println!("\n{}", serde_json::to_string_pretty(&stats).unwrap());
                    }
                } else {
                    println!("❌ Failed to fetch user stats");
                    if let Some(code) = result.code {
                        println!("  Error code: {:?}", code);
                    }
                }
            }
            Err(e) => {
                println!("❌ Error fetching user stats: {}", e);
            }
        }

        println!("\n=== Test Complete ===");
        println!("Note: This test made REAL API calls and did not modify any data");
    }
}
