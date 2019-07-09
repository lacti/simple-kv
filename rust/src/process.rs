use request::Request;
use response::Response;
use storage::Storage;

pub fn process(storage: &mut Storage, request: Request<'_>) -> Response {
    match request {
        Request::Get { key } => match storage.get(key) {
            Some(value) => Response::Value {
                value: value.clone(),
            },
            None => Response::Null,
        },
        Request::Set { key, value } => {
            storage.insert(key.to_string(), value.to_string());
            Response::Ok
        }
        Request::Remove { key } => {
            storage.remove(key);
            Response::Ok
        }
    }
}
