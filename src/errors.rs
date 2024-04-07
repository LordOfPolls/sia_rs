use reqwest;

#[derive(Debug)]
pub enum RequestError {
    RequestFailed(reqwest::Error),
    ParseFailed,
}
