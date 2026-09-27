#[derive(Debug)]
pub struct File {
    pub name: String,
    pub status: String,
    pub added: String,
    pub removed: String,
}

pub fn parse(line: &str) -> Result<File, String> {
    let mut file_line_to_parse = line.split('|').skip(1);

    let status = file_line_to_parse.next().ok_or("No Status")?;
    let name = file_line_to_parse.next().ok_or("No file name")?;
    let added = file_line_to_parse.next().ok_or("No added")?;
    let removed = file_line_to_parse.next().ok_or("No removed")?;

    Ok(File {
        name: name.to_string(),
        status: status.to_string(),
        added: added.to_string(),
        removed: removed.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_file_staged() {
        let line = "STAGED| M|.gitignore|+11|-1";

        let commit = parse(line).unwrap();

        assert_eq!(commit.status, " M");
        assert_eq!(commit.name, ".gitignore");
        assert_eq!(commit.added, "+11");
        assert_eq!(commit.removed, "-1");
    }

    #[test]
    fn parse_file_unstaged() {
        let line = "UNSTAGED|A |branch_data.rs|+11|-1";

        let commit = parse(line).unwrap();

        assert_eq!(commit.status, "A ");
        assert_eq!(commit.name, "branch_data.rs");
        assert_eq!(commit.added, "+11");
        assert_eq!(commit.removed, "-1");
    }

    #[test]
    fn parse_file_untracked() {
        let line = "UNTRACKED|??|locales/en.yml|+0|-0";

        let commit = parse(line).unwrap();

        assert_eq!(commit.status, "??");
        assert_eq!(commit.name, "locales/en.yml");
        assert_eq!(commit.added, "+0");
        assert_eq!(commit.removed, "-0");
    }
}
