#[derive(Debug)]
pub enum RequestError {
    RequestFailed(reqwest::Error),
    ParseFailed,
}

#[derive(Debug)]
pub enum ParseError {
    NoLicensesFound,
    TooManySearchResults,
    NoLicenseContainersFound
}