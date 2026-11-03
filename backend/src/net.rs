use reqwest::redirect::Policy;
use reqwest::{Client, Url};
use std::net::IpAddr;
use std::sync::OnceLock;

const MAX_REDIRECTS: usize = 4;

fn safe_client() -> &'static Client {
    static CLIENT: OnceLock<Client> = OnceLock::new();
    CLIENT.get_or_init(|| {
        Client::builder()
            .redirect(Policy::none())
            .timeout(std::time::Duration::from_secs(20))
            .user_agent("tranche-acceptance-agent/0.1")
            .build()
            .expect("safe HTTP client")
    })
}

pub struct Fetched {
    pub status: u16,
    pub body: String,
    pub final_url: Url,
}

/// Rejects URLs that cannot be part of a public evidence bundle and blocks
/// obvious SSRF targets (loopback, private ranges, link-local, metadata hosts).
pub fn validate_url_shape(raw: &str) -> Result<Url, String> {
    let url = Url::parse(raw.trim()).map_err(|_| "The URL is not valid.".to_string())?;
    match url.scheme() {
        "http" | "https" => {}
        _ => return Err("Only http and https URLs are accepted.".into()),
    }
    if url.username() != "" || url.password().is_some() {
        return Err("URLs with embedded credentials are not accepted.".into());
    }
    if raw.len() > 2048 {
        return Err("The URL is too long.".into());
    }
    let host = url
        .host_str()
        .ok_or_else(|| "The URL has no host.".to_string())?
        .to_ascii_lowercase();
    if host == "localhost"
        || host.ends_with(".localhost")
        || host.ends_with(".local")
        || host.ends_with(".internal")
        || host == "metadata.google.internal"
    {
        return Err("The URL points at a non-public host.".into());
    }
    if let Ok(ip) = host.parse::<IpAddr>() {
        if !is_public_ip(ip) {
            return Err("The URL points at a non-public address.".into());
        }
    }
    Ok(url)
}

async fn validate_resolved_host(url: &Url) -> Result<(), String> {
    let host = url
        .host_str()
        .ok_or_else(|| "The URL has no host.".to_string())?;
    if host.parse::<IpAddr>().is_ok() {
        return Ok(());
    }
    let port = url.port_or_known_default().unwrap_or(80);
    let addrs = tokio::net::lookup_host((host, port))
        .await
        .map_err(|_| "The URL host could not be resolved.".to_string())?;
    let mut seen = false;
    for addr in addrs {
        seen = true;
        if !is_public_ip(addr.ip()) {
            return Err("The URL resolves to a non-public address.".into());
        }
    }
    if !seen {
        return Err("The URL host did not resolve.".into());
    }
    Ok(())
}

fn is_public_ip(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => {
            !(v4.is_private()
                || v4.is_loopback()
                || v4.is_link_local()
                || v4.is_broadcast()
                || v4.is_documentation()
                || v4.is_unspecified()
                || v4.is_multicast())
        }
        IpAddr::V6(v6) => {
            !(v6.is_loopback()
                || v6.is_unspecified()
                || v6.is_multicast()
                || v6.is_unique_local()
                || v6.is_unicast_link_local())
        }
    }
}

/// Fetches a public URL with manual redirect handling and a hard body cap.
pub async fn fetch_public(raw: &str, max_bytes: usize) -> Result<Fetched, String> {
    let mut current = validate_url_shape(raw)?;
    let client = safe_client();
    for _ in 0..MAX_REDIRECTS {
        validate_resolved_host(&current).await?;
        let response = client
            .get(current.clone())
            .timeout(std::time::Duration::from_secs(15))
            .send()
            .await
            .map_err(|e| format!("The URL could not be reached: {e}"))?;

        let status = response.status();
        if status.is_redirection() {
            let location = response
                .headers()
                .get(reqwest::header::LOCATION)
                .and_then(|value| value.to_str().ok())
                .ok_or_else(|| "The URL redirected without a location.".to_string())?;
            current = current
                .join(location)
                .map_err(|_| "The redirect target is invalid.".to_string())?;
            continue;
        }

        let mut body = String::new();
        let mut response = response;
        let mut total = 0usize;
        while let Some(chunk) = response
            .chunk()
            .await
            .map_err(|e| format!("The response body could not be read: {e}"))?
        {
            total += chunk.len();
            let remaining = max_bytes.saturating_sub(body.len());
            if remaining == 0 {
                break;
            }
            let take = chunk.len().min(remaining);
            body.push_str(&String::from_utf8_lossy(&chunk[..take]));
            if total > max_bytes {
                break;
            }
        }

        return Ok(Fetched {
            status: status.as_u16(),
            body,
            final_url: current,
        });
    }
    Err("The URL redirected too many times.".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blocks_localhost_and_private_ranges() {
        assert!(validate_url_shape("http://localhost:8080/secret").is_err());
        assert!(validate_url_shape("http://127.0.0.1/").is_err());
        assert!(validate_url_shape("http://10.0.0.4/").is_err());
        assert!(validate_url_shape("http://169.254.169.254/latest/meta-data").is_err());
        assert!(validate_url_shape("javascript:alert(1)").is_err());
        assert!(validate_url_shape("https://example.com/demo").is_ok());
    }
}
