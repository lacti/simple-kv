pub enum Request<'a> {
    Get { key: &'a str },
    Set { key: &'a str, value: &'a str },
    Remove { key: &'a str },
}

impl<'a> Request<'a> {
    pub fn parse(input: &'a str) -> Result<Self, &'static str> {
        let mut parts = input.split_ascii_whitespace();
        let command = parts.next();
        if command.is_none() {
            return Err("empty input");
        }
        let command = command.unwrap().to_ascii_lowercase();

        match command.as_str() {
            "get" => {
                let key = match parts.next() {
                    Some(key) => key,
                    None => return Err("GET must be followed by a key"),
                };
                Ok(Request::Get { key: key })
            }
            "set" => {
                let key = match parts.next() {
                    Some(key) => key,
                    None => return Err("SET must be followed by a key"),
                };
                let value = match parts.next() {
                    Some(value) => value,
                    None => return Err("SET needs a value"),
                };
                Ok(Request::Set {
                    key: key,
                    value: value,
                })
            }
            "remove" => {
                let key = match parts.next() {
                    Some(key) => key,
                    None => return Err("GET must be followed by a key"),
                };
                Ok(Request::Remove { key: key })
            }
            _ => Err("unknown command"),
        }
    }
}
