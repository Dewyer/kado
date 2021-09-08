pub enum Authorizer {
    Google,
}

impl Authorizer {
    pub fn from_string(value: String) -> anyhow::Result<Self> {
        Ok(match &value.to_lowercase()[..] {
            "google" => Authorizer::Google,
            _ => bail!("Invalid authorizer!"),
        })
    }
}