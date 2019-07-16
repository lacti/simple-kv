pub enum Response {
    Ok,
    Value {
        value: String,
    },
    Null,
    #[allow(dead_code)]
    None,
}

impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Response::Ok => writeln!(f, "OK"),
            Response::Value { value } => writeln!(f, "{}", &value),
            Response::Null => writeln!(f, "NULL"),
            Response::None => Ok(()),
        }
    }
}
