//! Discord OAuth authentication flow.
//!
//! This module handles the Discord sign-in flow by:
//! 1. Starting a temporary localhost HTTP server on a random port
//! 2. Requesting an OAuth URL from the API with the port
//! 3. Opening the Discord authorization URL in the user's browser
//! 4. Waiting for the callback with the verification token
//! 5. Emitting the token to the frontend via Tauri events

use crate::auth::error::AuthError;
use crate::auth::API_BASE_URL;
use serde::{Deserialize, Serialize};
use std::net::TcpListener as StdTcpListener;
use tauri::{AppHandle, Emitter};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

/// Response from the launcher-bridge endpoint
#[derive(Debug, Deserialize)]
struct LauncherBridgeResponse {
    url: String,
}

/// Payload emitted when Discord auth completes
#[derive(Debug, Clone, Serialize)]
pub struct DiscordAuthResult {
    pub success: bool,
    pub token: Option<String>,
    pub error: Option<String>,
}

/// Finds a random available port by binding to port 0.
fn find_available_port() -> Result<u16, AuthError> {
    let listener = StdTcpListener::bind("127.0.0.1:0")
        .map_err(|e| AuthError::Unknown(format!("Failed to find available port: {}", e)))?;
    let port = listener
        .local_addr()
        .map_err(|e| AuthError::Unknown(format!("Failed to get local address: {}", e)))?
        .port();
    // Listener is dropped here, freeing the port
    Ok(port)
}

/// Parses an HTTP request and extracts query parameters.
fn parse_query_params(request: &str) -> Option<(Option<String>, Option<String>)> {
    // Find the GET line
    let first_line = request.lines().next()?;
    if !first_line.starts_with("GET ") {
        return None;
    }

    // Extract the path with query string
    let path = first_line.strip_prefix("GET ")?.split_whitespace().next()?;

    // Find query string
    let query = path.split('?').nth(1)?;

    let mut token_type: Option<String> = None;
    let mut token: Option<String> = None;

    for param in query.split('&') {
        if let Some((key, value)) = param.split_once('=') {
            match key {
                "type" => token_type = Some(urlencoding_decode(value)),
                "token" => token = Some(urlencoding_decode(value)),
                _ => {}
            }
        }
    }

    Some((token_type, token))
}

/// Simple URL decoding for the token value.
fn urlencoding_decode(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '%' {
            let hex: String = chars.by_ref().take(2).collect();
            if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                result.push(byte as char);
            } else {
                result.push('%');
                result.push_str(&hex);
            }
        } else if c == '+' {
            result.push(' ');
        } else {
            result.push(c);
        }
    }

    result
}

/// Generates an HTTP redirect response.
fn generate_redirect_response() -> String {
    [
        "HTTP/1.1 302 Found",
        "Location: https://duelsplus.com/launcher-bridge",
        "Connection: close",
        "",
        "",
    ]
    .join("\r\n")
}

/// Starts the Discord OAuth flow.
///
/// This function:
/// 1. Starts a temporary localhost HTTP server on a random port
/// 2. Requests the Discord OAuth URL from the API
/// 3. Opens the URL in the user's browser
/// 4. Waits for the callback with the verification token
/// 5. Emits the token to the frontend via a Tauri event
///
/// # Arguments
///
/// * `app` - The Tauri app handle for opening URLs and emitting events
///
/// # Returns
///
/// Returns `Ok(())` if the flow started successfully, or an error if something went wrong.
/// The actual token will be delivered via the `discord-auth-result` event.
pub async fn start_discord_signin(app: AppHandle) -> Result<(), AuthError> {
    // Find an available port
    let port = find_available_port()?;

    // Start the TCP listener
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .map_err(|e| AuthError::Unknown(format!("Failed to start local server: {}", e)))?;

    // Request the OAuth URL from the API
    let client = reqwest::Client::new();
    let url = format!("{}/auth/launcher-bridge?port={}", API_BASE_URL, port);

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| AuthError::Network(e))?;

    if !response.status().is_success() {
        return Err(AuthError::Unknown(format!(
            "API returned status {}",
            response.status()
        )));
    }

    let bridge_response: LauncherBridgeResponse =
        response.json().await.map_err(|e| AuthError::Network(e))?;

    // Open the Discord OAuth URL in the browser
    if let Err(e) = open::that(&bridge_response.url) {
        let _ = app.emit(
            "discord-auth-result",
            DiscordAuthResult {
                success: false,
                token: None,
                error: Some(format!("Failed to open browser: {}", e)),
            },
        );
        return Err(AuthError::Unknown(format!("Failed to open browser: {}", e)));
    }

    // Spawn a task to handle the callback
    let app_clone = app.clone();
    tokio::spawn(async move {
        // Set a timeout for the authentication
        let timeout = tokio::time::timeout(
            tokio::time::Duration::from_secs(300), // 5 minute timeout
            wait_for_callback(listener),
        )
        .await;

        match timeout {
            Ok(Ok(token)) => {
                let _ = app_clone.emit(
                    "discord-auth-result",
                    DiscordAuthResult {
                        success: true,
                        token: Some(token),
                        error: None,
                    },
                );
            }
            Ok(Err(e)) => {
                let _ = app_clone.emit(
                    "discord-auth-result",
                    DiscordAuthResult {
                        success: false,
                        token: None,
                        error: Some(e),
                    },
                );
            }
            Err(_) => {
                let _ = app_clone.emit(
                    "discord-auth-result",
                    DiscordAuthResult {
                        success: false,
                        token: None,
                        error: Some("Authentication timed out".to_string()),
                    },
                );
            }
        }
    });

    Ok(())
}

/// Waits for the OAuth callback on the local server.
async fn wait_for_callback(listener: TcpListener) -> Result<String, String> {
    // Accept a connection
    let (mut socket, _) = listener
        .accept()
        .await
        .map_err(|e| format!("Failed to accept connection: {}", e))?;

    // Read the HTTP request
    let mut buffer = vec![0u8; 4096];
    let n = socket
        .read(&mut buffer)
        .await
        .map_err(|e| format!("Failed to read request: {}", e))?;

    let request = String::from_utf8_lossy(&buffer[..n]);

    // Parse the query parameters
    let (token_type, token) =
        parse_query_params(&request).ok_or_else(|| "Invalid request format".to_string())?;

    // Validate the response
    let success = matches!(
        (token_type.as_deref(), &token),
        (Some("token"), Some(t)) if !t.is_empty()
    );

    // Send redirect response
    let http_response = generate_redirect_response();
    let _ = socket.write_all(http_response.as_bytes()).await;
    let _ = socket.flush().await;

    if success {
        Ok(token.unwrap())
    } else {
        Err("Authentication failed or was cancelled".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_query_params_valid() {
        let request = "GET /?type=token&token=abc123 HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let result = parse_query_params(request);
        assert!(result.is_some());
        let (token_type, token) = result.unwrap();
        assert_eq!(token_type, Some("token".to_string()));
        assert_eq!(token, Some("abc123".to_string()));
    }

    #[test]
    fn test_parse_query_params_error_type() {
        let request = "GET /?type=error HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let result = parse_query_params(request);
        assert!(result.is_some());
        let (token_type, token) = result.unwrap();
        assert_eq!(token_type, Some("error".to_string()));
        assert_eq!(token, None);
    }

    #[test]
    fn test_parse_query_params_url_encoded() {
        let request = "GET /?type=token&token=abc%20123%2B456 HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let result = parse_query_params(request);
        assert!(result.is_some());
        let (token_type, token) = result.unwrap();
        assert_eq!(token_type, Some("token".to_string()));
        assert_eq!(token, Some("abc 123+456".to_string()));
    }

    #[test]
    fn test_parse_query_params_invalid_request() {
        let request = "POST / HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let result = parse_query_params(request);
        assert!(result.is_none());
    }

    #[test]
    fn test_urlencoding_decode() {
        assert_eq!(urlencoding_decode("hello%20world"), "hello world");
        assert_eq!(urlencoding_decode("test+value"), "test value");
        assert_eq!(urlencoding_decode("abc%2F123"), "abc/123");
        assert_eq!(urlencoding_decode("plain"), "plain");
    }

    #[test]
    fn test_find_available_port() {
        let port = find_available_port().unwrap();
        assert!(port > 0);
        // Verify the port is in a reasonable range
        assert!(port > 1000);
    }
}
