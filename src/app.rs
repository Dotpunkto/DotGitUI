use crate::parse;
use std::process::Command;

pub struct App {
    pub branch: parse::branch::Branch,
    pub updated_files: Vec<String>,
    pub last_commits: Vec<parse::commit::Commit>,
}

pub fn load_app() -> Result<App, String> {
    let status = Command::new("git")
        .args(["status", "--porcelain=v1", "--branch"])
        .output()
        .map_err(|e| format!("git status failed: {e}"))?;
    let status_out = String::from_utf8_lossy(&status.stdout);

    let branch = parse::branch::parse(&status_out)?;

    let updated_files: Vec<String> = Vec::with_capacity(3);

    let log = Command::new("git")
        .args([
            "log",
            "-n",
            "5",
            "--date=short",
            "--pretty=format:%h\t%an\t%ad\t%s",
        ])
        .output()
        .map_err(|e| format!("git log failed: {e}"))?;

    let log_out = String::from_utf8_lossy(&log.stdout);
    let last_commits: Result<Vec<parse::commit::Commit>, String> = log_out
        .lines()
        .filter(|l| !l.is_empty())
        .map(parse::commit::parse)
        .collect();

    Ok(App {
        branch,
        updated_files,
        last_commits: last_commits?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
}
