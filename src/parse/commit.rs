#[derive(Debug)]
pub struct Commit {
    pub short_hash: String,
    pub message: String,
    pub author: String,
    pub date: String,
}

pub fn parse(line: &str) -> Result<Commit, String> {
    if !line.contains('\t') {
        return Err("Tab separator is missing".to_string());
    }

    let mut commit_to_parce = line.split('\t');

    let short_hash = commit_to_parce.next().ok_or("Hash is missing")?;

    if short_hash.is_empty() {
        return Err("Hash is missing".to_string());
    }

    let author = commit_to_parce.next().ok_or("Author is missing")?;

    if author.is_empty() {
        return Err("Author is missing".to_string());
    }

    let date = commit_to_parce.next().ok_or("Date is missing")?;

    if date.is_empty() {
        return Err("Date is missing".to_string());
    }

    let message = commit_to_parce.next().ok_or("Message is missing")?;

    if message.is_empty() {
        return Err("Message is missing".to_string());
    }

    Ok(Commit {
        short_hash: short_hash.to_string(),
        message: message.to_string(),
        author: author.to_string(),
        date: date.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_commit_valide() {
        let line = "abcd123\tJohn Doe\t2026-09-06\tGit init";

        let commit = parse(line).unwrap();

        assert_eq!(commit.short_hash, "abcd123");
        assert_eq!(commit.author, "John Doe");
        assert_eq!(commit.date, "2026-09-06");
        assert_eq!(commit.message, "Git init");
    }

    #[test]
    fn parse_commit_without_tabulation() {
        let line = "abcd123 John Doe 2026-09-06 Git init";

        let error = parse(line).unwrap_err();

        assert_eq!(error, "Tab separator is missing");
    }

    #[test]
    fn parse_commit_hash_missing() {
        let line = "\tJohn Doe\t2026-09-06\tGit init";

        let error = parse(line).unwrap_err();

        assert_eq!(error, "Hash is missing");
    }

    #[test]
    fn parse_commit_author_missing() {
        let line = "abcd123\t\t2026-09-06\tGit init";

        let error = parse(line).unwrap_err();

        assert_eq!(error, "Author is missing");
    }

    #[test]
    fn parse_commit_date_missing() {
        let line = "abcd123\tJohn Doe\t\tGit init";

        let error = parse(line).unwrap_err();

        assert_eq!(error, "Date is missing");
    }

    #[test]
    fn parse_commit_message_missing() {
        let line = "abcd123\tJohn Doe\t2026-09-06\t";

        let error = parse(line).unwrap_err();

        assert_eq!(error, "Message is missing");
    }
}
