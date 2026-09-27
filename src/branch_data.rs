use crate::parse;
use std::process::Command;

pub struct BranchData {
    pub branch: parse::branch::Branch,
    pub updated_staged: Vec<parse::file::File>,
    pub updated_unstaged: Vec<parse::file::File>,
    pub updated_untracked: Vec<parse::file::File>,
    pub last_commits: Vec<parse::commit::Commit>,
}

pub fn load_branch_data() -> Result<BranchData, String> {
    let git_branch_command = parse::command::parse("git branch --show-current")?;
    let branch = Command::new(git_branch_command.name)
        .args(git_branch_command.args)
        .output()
        .map_err(|e| format!("git branch failed: {e}"))?
        .stdout;

    let updated_files_command = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src/commands/get_updated_files.sh");
    let updated_files = Command::new("sh")
        .arg(&updated_files_command)
        .output()
        .map_err(|e| format!("Get updated files : {e}"))?
        .stdout;

    let updated_staged = String::from_utf8_lossy(&updated_files)
        .lines()
        .filter(|l| l.starts_with("STAGED") && !l.is_empty())
        .map(parse::file::parse)
        .collect::<Result<Vec<parse::file::File>, String>>()?;

    let updated_unstaged = String::from_utf8_lossy(&updated_files)
        .lines()
        .filter(|l| l.starts_with("UNSTAGED") && !l.is_empty())
        .map(parse::file::parse)
        .collect::<Result<Vec<parse::file::File>, String>>()?;

    let updated_untracked = String::from_utf8_lossy(&updated_files)
        .lines()
        .filter(|l| l.starts_with("UNTRACKED") && !l.is_empty())
        .map(parse::file::parse)
        .collect::<Result<Vec<parse::file::File>, String>>()?;

    let git_log_command =
        parse::command::parse("git log -n 5 --date=short --pretty=format:%h\t%an\t%ad\t%s")?;
    let log = Command::new(git_log_command.name)
        .args(git_log_command.args)
        .output()
        .map_err(|e| format!("git log failed: {e}"))?
        .stdout;

    let log_out = String::from_utf8_lossy(&log);
    let last_commits: Result<Vec<parse::commit::Commit>, String> = log_out
        .lines()
        .filter(|l| !l.is_empty())
        .map(parse::commit::parse)
        .collect();

    Ok(BranchData {
        branch: parse::branch::Branch {
            name: String::from_utf8_lossy(&branch).to_string(),
        },
        updated_staged: updated_staged,
        updated_unstaged: updated_unstaged,
        updated_untracked: updated_untracked,
        last_commits: last_commits?,
    })
}
