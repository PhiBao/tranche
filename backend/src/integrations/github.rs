use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct RepoInfo {
    pub full_name: String,
    pub private: bool,
    pub description: Option<String>,
    pub default_branch: String,
    pub stars: i64,
}

#[derive(Debug, Deserialize)]
struct GithubRepo {
    full_name: String,
    private: bool,
    description: Option<String>,
    default_branch: String,
    stargazers_count: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct GithubReadme {
    content: Option<String>,
    encoding: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GithubCommit {
    sha: String,
    commit: GithubCommitInner,
}

#[derive(Debug, Deserialize)]
struct GithubCommitInner {
    message: String,
    author: GithubAuthor,
}

#[derive(Debug, Deserialize)]
struct GithubAuthor {
    date: String,
}

pub fn parse_repo_url(url: &str) -> Option<(String, String)> {
    let trimmed = url.trim().trim_end_matches('/');
    let rest = trimmed
        .strip_prefix("https://github.com/")
        .or_else(|| trimmed.strip_prefix("http://github.com/"))
        .or_else(|| trimmed.strip_prefix("github.com/"))?;
    let mut parts = rest.split('/');
    let owner = parts.next()?.to_string();
    let repo = parts.next()?.trim_end_matches(".git").to_string();
    if owner.is_empty() || repo.is_empty() {
        return None;
    }
    let valid = |value: &str| {
        value
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
    };
    if !valid(&owner) || !valid(&repo) || owner == "." || owner == ".." || repo == "." || repo == ".." {
        return None;
    }
    Some((owner, repo))
}

fn user_agent() -> String {
    "tranche-acceptance-agent/0.1 (+https://github.com/tranche-build)".into()
}

pub async fn fetch_repo(
    http: &Client,
    token: Option<&str>,
    owner: &str,
    repo: &str,
) -> Result<RepoInfo, String> {
    let url = format!("https://api.github.com/repos/{owner}/{repo}");
    let mut request = http.get(url).header("User-Agent", user_agent()).header(
        "Accept",
        "application/vnd.github+json",
    );
    if let Some(token) = token {
        request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await.map_err(|e| format!("GitHub request failed: {e}"))?;
    let status = response.status();
    if status.as_u16() == 404 {
        return Err("Repository not found or not public".into());
    }
    if status.as_u16() == 403 || status.as_u16() == 429 {
        return Err("GitHub rate limit reached. Configure GITHUB_TOKEN to raise the limit.".into());
    }
    if !status.is_success() {
        return Err(format!("GitHub returned HTTP {status}"));
    }
    let body: GithubRepo = response
        .json()
        .await
        .map_err(|e| format!("GitHub returned an unexpected response: {e}"))?;
    Ok(RepoInfo {
        full_name: body.full_name,
        private: body.private,
        description: body.description,
        default_branch: body.default_branch,
        stars: body.stargazers_count.unwrap_or(0),
    })
}

pub async fn fetch_readme(
    http: &Client,
    token: Option<&str>,
    owner: &str,
    repo: &str,
    explicit_url: &str,
) -> Result<String, String> {
    if !explicit_url.trim().is_empty() {
        let fetched = crate::net::fetch_public(explicit_url, 2_000_000).await?;
        if !(200..300).contains(&fetched.status) {
            return Err(format!("README URL returned HTTP {}", fetched.status));
        }
        return Ok(fetched.body);
    }

    let url = format!("https://api.github.com/repos/{owner}/{repo}/readme");
    let mut request = http.get(url).header("User-Agent", user_agent()).header(
        "Accept",
        "application/vnd.github+json",
    );
    if let Some(token) = token {
        request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await.map_err(|e| format!("GitHub request failed: {e}"))?;
    let status = response.status();
    if !status.is_success() {
        return Err(format!("README not found (HTTP {status})"));
    }
    let body: GithubReadme = response
        .json()
        .await
        .map_err(|e| format!("GitHub returned an unexpected README response: {e}"))?;
    let content = body.content.ok_or("README content missing")?;
    if body.encoding.as_deref() != Some("base64") {
        return Ok(content);
    }
    let cleaned: String = content.chars().filter(|c| !c.is_whitespace()).collect();
    let bytes = base64_decode(&cleaned)?;
    String::from_utf8(bytes).map_err(|_| "README is not valid UTF-8".to_string())
}

pub async fn fetch_latest_commit(
    http: &Client,
    token: Option<&str>,
    owner: &str,
    repo: &str,
) -> Result<(String, String, String), String> {    let url = format!("https://api.github.com/repos/{owner}/{repo}/commits?per_page=1");
    let mut request = http.get(url).header("User-Agent", user_agent()).header(
        "Accept",
        "application/vnd.github+json",
    );
    if let Some(token) = token {
        request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await.map_err(|e| format!("GitHub request failed: {e}"))?;
    if !response.status().is_success() {
        return Err(format!("GitHub commits returned HTTP {}", response.status()));
    }
    let body: Vec<GithubCommit> = response
        .json()
        .await
        .map_err(|e| format!("GitHub returned an unexpected commits response: {e}"))?;
    let first = body.first().ok_or("Repository has no commits")?;
    Ok((
        first.sha.clone(),
        first.commit.message.lines().next().unwrap_or("").to_string(),
        first.commit.author.date.clone(),
    ))
}

/// Returns (status, conclusion) pairs for check runs on the latest commit,
/// e.g. ("completed","success") or ("in_progress", None).
pub async fn fetch_latest_check_runs(
    http: &Client,
    token: Option<&str>,
    owner: &str,
    repo: &str,
) -> Result<Vec<(String, Option<String>)>, String> {
    let (sha, _, _) = fetch_latest_commit(http, token, owner, repo).await?;
    let url = format!(
        "https://api.github.com/repos/{owner}/{repo}/commits/{sha}/check-runs?per_page=20"
    );
    let mut request = http.get(url).header("User-Agent", user_agent()).header(
        "Accept",
        "application/vnd.github+json",
    );
    if let Some(token) = token {
        request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await.map_err(|e| format!("GitHub request failed: {e}"))?;
    if !response.status().is_success() {
        return Err(format!("GitHub check-runs returned HTTP {}", response.status()));
    }
    #[derive(Debug, Deserialize)]
    struct CheckRunsBody {
        #[serde(default)]
        check_runs: Vec<CheckRun>,
    }
    #[derive(Debug, Deserialize)]
    struct CheckRun {
        #[serde(default)]
        status: Option<String>,
        #[serde(default)]
        conclusion: Option<String>,
    }
    let body: CheckRunsBody = response
        .json()
        .await
        .map_err(|e| format!("GitHub returned an unexpected check-runs response: {e}"))?;
    Ok(body
        .check_runs
        .into_iter()
        .map(|run| (run.status.unwrap_or_default(), run.conclusion))
        .collect())
}

fn base64_decode(input: &str) -> Result<Vec<u8>, String> {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut lookup = [255u8; 256];
    for (i, byte) in TABLE.iter().enumerate() {
        lookup[*byte as usize] = i as u8;
    }
    let mut out = Vec::with_capacity(input.len() * 3 / 4);
    let mut buffer: u32 = 0;
    let mut bits = 0;
    for byte in input.bytes() {
        if byte == b'=' {
            break;
        }
        let value = lookup[byte as usize];
        if value == 255 {
            return Err("README base64 contains an invalid character".into());
        }
        buffer = (buffer << 6) | value as u32;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((buffer >> bits) as u8);
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_repo_urls() {
        assert_eq!(
            parse_repo_url("https://github.com/openserv-labs/sdk"),
            Some(("openserv-labs".into(), "sdk".into()))
        );
        assert_eq!(
            parse_repo_url("https://github.com/openserv-labs/sdk.git/"),
            Some(("openserv-labs".into(), "sdk".into()))
        );
        assert_eq!(parse_repo_url("https://example.com/foo/bar"), None);
    }
}
