use std::{os::windows::process::CommandExt, path::Path, process::Command};

const CREATE_NO_WINDOW: u32 = 0x0800_0000;

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

/// git@host:owner/repo.git
/// ssh://git@host:22/owner/repo.git
/// https://user:token@host/owner/repo.git
/// all become
/// https://host/owner/repo
fn web_url(remote: &str) -> Option<String> {
    let remote = remote.strip_suffix(".git").unwrap_or(remote);

    let (host, path) = match remote.split_once("://") {
        Some((_, rest)) => rest.split_once('/')?,
        None => remote.split_once(':')?,
    };

    let host = host.rsplit('@').next()?;
    let host = host.split(':').next()?;

    Some(format!("https://{host}/{path}"))
}
