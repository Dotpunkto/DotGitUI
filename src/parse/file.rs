#[derive(Debug)]
pub struct File {
    pub name: String,
    pub status: String,
}

pub fn parse(line: &str) -> Result<File, String> {
    let mut file_line_to_parse = line.trim().split(' ');

    let status = file_line_to_parse.next().ok_or("No Status")?;
    let name = file_line_to_parse.next().ok_or("No file name")?;

    Ok(File {
        name: name.to_string(),
        status: status.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_file_valide() {
        let line = " M .gitignore";

        let commit = parse(line).unwrap();

        assert_eq!(commit.status, "M");
        assert_eq!(commit.name, ".gitignore");
    }

    #[test]
    fn parse_untracked_file_valide() {
        let line = "?? locales/en.yml";

        let commit = parse(line).unwrap();

        assert_eq!(commit.status, "??");
        assert_eq!(commit.name, "locales/en.yml");
    }
}
