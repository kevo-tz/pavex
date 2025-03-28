use pavex::cookie::SameSite;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
/// Configure the cookie used to store session information on the client-side.
pub struct SessionCookieConfig {
    /// The name of the cookie used to store the session ID.
    ///
    /// By default, the name is set to `id`.
    #[serde(default = "default_session_cookie_name")]
    pub name: String,
    /// Set the `Domain` attribute on the cookie used to store the session ID.
    ///
    /// By default, the attribute is not set.
    #[serde(default)]
    pub domain: Option<String>,
    /// Set the `Path` attribute on the cookie used to store the session ID.
    ///
    /// By default, the attribute is set to `/`.
    #[serde(default = "default_session_cookie_path")]
    pub path: Option<String>,
    /// Set the `Secure` attribute on the cookie used to store the session ID.
    ///
    /// If the cookie is marked as `Secure`, it will only be transmitted when the connection is secure (e.g. over HTTPS).
    ///
    /// Default is `true`.
    #[serde(default = "default_session_cookie_secure")]
    pub secure: bool,
    /// Set the `HttpOnly` attribute on the cookie used to store the session ID.
    ///
    /// If the cookie is marked as `HttpOnly`, it will not be visible to JavaScript
    /// snippets running in the browser.
    ///
    /// Default is `true`.
    #[serde(default = "default_session_cookie_http_only")]
    pub http_only: bool,
    /// Set the [`SameSite`] attribute on the cookie used to store the session ID.
    ///
    /// By default, the attribute is set to [`SameSite::Lax`].
    #[serde(default = "default_session_cookie_same_site")]
    pub same_site: Option<SameSite>,
    /// The kind of session cookie to use.
    ///
    /// By default, it is set to [`SessionCookieKind::Persistent`].
    #[serde(default)]
    pub kind: SessionCookieKind,
}

impl Default for SessionCookieConfig {
    fn default() -> Self {
        Self {
            name: default_session_cookie_name(),
            domain: None,
            path: default_session_cookie_path(),
            secure: default_session_cookie_secure(),
            http_only: default_session_cookie_http_only(),
            same_site: default_session_cookie_same_site(),
            kind: Default::default(),
        }
    }
}

fn default_session_cookie_name() -> String {
    // See https://cheatsheetseries.owasp.org/cheatsheets/Session_Management_Cheat_Sheet.html#session-id-name-fingerprinting
    "id".to_string()
}

fn default_session_cookie_secure() -> bool {
    true
}

fn default_session_cookie_http_only() -> bool {
    true
}

fn default_session_cookie_path() -> Option<String> {
    Some("/".to_string())
}

fn default_session_cookie_same_site() -> Option<SameSite> {
    Some(SameSite::Lax)
}

/// The kind of cookie used to store session information on the client-side.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum SessionCookieKind {
    /// A persistent session cookie.
    ///
    /// The cookie will be stored on the client's device with an
    /// expiration date set by the server via the `Max-Age` attribute.
    ///
    /// This is the default.
    #[default]
    Persistent,
    /// A cookie that expires when the browser session ends.
    ///
    /// Each browser has its own concept of "browser session", e.g. the session
    /// doesn't necessarily end when the browser window or tab is closed.
    /// For example, both Firefox and Chrome automatically restore the session
    /// when the browser is restarted, keeping all session cookies alive.
    /// Consider using [`SessionCookieKind::Persistent`]
    /// if you don't want to deal with the nuances of browser-specific behaviour.
    Session,
}
