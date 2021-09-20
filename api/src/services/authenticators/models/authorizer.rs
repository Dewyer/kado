pub enum Authorizer {
    Google,
    Github,
}

impl Authorizer {
    pub fn from_string(value: String) -> anyhow::Result<Self> {
        Ok(match &value.to_lowercase()[..] {
            "google" => Authorizer::Google,
            "github" => Authorizer::Github,
            _ => bail!("Invalid authorizer!"),
        })
    }
}