// #[derive(Debug)]
// pub enum RequestError {
//     RequestFailed(reqwest::Error),
//     ParseFailed,
// }
//
// #[derive(Debug)]
// pub enum ParseError {
//     NoLicensesFound,
//     TooManySearchResults,
//     NoLicenseContainersFound,
// }

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SIAError {
    #[error("Request failed: {0}")]
    Error(String),

    #[error("No licenses found.")]
    NoLicensesFound,
    #[error("Too many search results.")]
    TooManyResults,
    #[error("Unable to parse license data.")]
    ParseFailed,

    #[error("Request failed: {0}")]
    RequestFailed(reqwest::Error),
}
