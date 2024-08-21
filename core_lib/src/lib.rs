#[cfg(feature = "server")]
pub mod server;

pub enum RequestMethod {
    POST,
    GET
}

pub struct Request{
    pub method: RequestMethod,
    pub path: Option<String> 
}

