#[derive(Debug)]
pub struct File {
    pub name: String,
}

pub fn parse(line: &str) -> Result<File, String> {
    let branch = String::new();

    Ok(File { name: branch })
}

#[cfg(test)]
mod tests {
    use super::*;
}
