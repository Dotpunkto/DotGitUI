#[derive(Debug)]
pub struct Branch {
    pub name: String,
}

pub fn parse(line: &str) -> Result<Branch, String> {
    let branch = line
        .lines()
        .next()
        .and_then(|l| l.strip_prefix("## "))
        .and_then(|l| l.split("...").next())
        .ok_or_else(|| format!("Branch : parssing error\n Value: {line}"))?
        .to_string();

    Ok(Branch { name: branch })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_branch_valide() {
        let line = "## main...origin/main";

        let commit = parse(line).unwrap();

        assert_eq!(commit.name, "main");
    }

    #[test]
    fn parse_branch_invalide() {
        let line = " ...origin/main";

        let error = parse(line).unwrap_err();

        assert_eq!(error, "Branch : parssing error");
    }
}
