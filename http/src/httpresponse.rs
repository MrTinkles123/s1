#[derive(Debug, PartialEq!)]

pub enum Method {
    Get,
    Post,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(s) -> Self {
        match s {
            "Get" => Method::Get,
            "Post" => Method::Post,
            _ => Method::Uninitialized,
        }
    }
}