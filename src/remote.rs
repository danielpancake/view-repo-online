use std::{os::windows::process::CommandExt, path::Path, process::Command};

const CREATE_NO_WINDOW: u32 = 0x0800_0000;

/// Whether `folder` is inside a Git repository
/// Only checks for `.git`, so it's fast enough to run on every right-click
pub fn is_repository(folder: &Path) -> bool {
    folder.ancestors().any(|dir| dir.join(".git").exists())
}

/// Returns the web page of the repository containing `folder`
pub fn repository_url(folder: &Path) -> Result<String, String> {
    let remote = git(folder, &["remote", "get-url", "origin"])?;
    web_url(&remote).ok_or_else(|| format!("No web page known for {remote}"))
}

/// Runs Git in `folder` and returns its output, or its error message
fn git(folder: &Path, args: &[&str]) -> Result<String, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(folder)
        .args(args)
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .map_err(|_| "Git is not installed.")?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().into());
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().into())
}

/// git@host:owner/repo.git                   -> https://host/owner/repo
/// ssh://git@host:22/owner/repo.git          -> https://host/owner/repo
/// https://user:token@host:8443/owner/repo   -> https://host:8443/owner/repo
/// git@ssh.dev.azure.com:v3/org/project/repo -> https://dev.azure.com/org/project/_git/repo
/// TODO: probably better way to handle
fn web_url(remote: &str) -> Option<String> {
    let remote = remote.strip_suffix(".git").unwrap_or(remote);

    let (scheme, host, path) = match remote.split_once("://") {
        Some((scheme, rest)) => {
            let (host, path) = rest.split_once('/')?;
            (scheme, host, path)
        }
        None => {
            let (host, path) = remote.split_once(':')?;
            ("ssh", host, path)
        }
    };

    let host = host.rsplit('@').next()?;

    // HTTP(S) remotes already point at the website, port and all
    if scheme == "http" || scheme == "https" {
        return Some(format!("{scheme}://{host}/{path}"));
    }

    let host = host.split(':').next()?;

    // Azure DevOps SSH paths look like v3/org/project/repo
    if host == "ssh.dev.azure.com" || host.ends_with("vs-ssh.visualstudio.com") {
        let (org, rest) = path.strip_prefix("v3/")?.split_once('/')?;
        let (project, repo) = rest.split_once('/')?;
        return Some(format!("https://dev.azure.com/{org}/{project}/_git/{repo}"));
    }

    Some(format!("https://{host}/{path}"))
}
