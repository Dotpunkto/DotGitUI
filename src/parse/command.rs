#[derive(Debug)]
pub struct Command {
    pub name: String,
    pub args: Vec<String>,
}

pub fn parse(line: &str) -> Result<Command, String> {
    let first_line = line
        .lines()
        .find(|l| !l.is_empty())
        .ok_or("No command found")?;

    let mut parts = first_line.split(" ");

    let name = parts.next().ok_or("Command name not found")?;

    let args = parts.map(|l| l.to_string()).collect();

    Ok(Command {
        name: name.to_string(),
        args: args,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_command_valide() {
        let line = "git status --porcelain=v1 --branch";

        let command = parse(line).unwrap();

        assert_eq!(command.name, "git");
        assert_eq!(command.args, vec!["status", "--porcelain=v1", "--branch"]);
    }

    #[test]
    fn parse_command_not_found() {
        let line = "";

        let err = parse(line).unwrap_err();

        assert_eq!(err, "No command found");
    }

    #[test]
    fn parse_complex_command() {
        let line = "git log -n 5 --date=short --pretty=format:%h\t%an\t%ad\t%s";

        let command = parse(line).unwrap();

        assert_eq!(command.name, "git");
        assert_eq!(
            command.args,
            vec![
                "log",
                "-n",
                "5",
                "--date=short",
                "--pretty=format:%h\t%an\t%ad\t%s"
            ]
        );
    }
}
