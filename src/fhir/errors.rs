#[derive(Debug)]
pub enum ParseError {
    InvalidJSON(serde_json::Error),
    MalformedBundle,
}
