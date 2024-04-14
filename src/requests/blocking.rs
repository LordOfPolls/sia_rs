// blocking variant of the request functions

use std::time::Duration;

use log::{debug, error, warn};
use reqwest::blocking::Client;

use crate::errors::{ParseError, RequestError};
use crate::models::payloads::{SearchByLicense, SearchByName};
use crate::models::LicenseState;
use crate::requests::parsers::parse;
use crate::RequestError::ParseFailed;
use crate::{SEARCH_LICENSE_NUM_URL, SEARCH_NAME_URL};

/// Base function for making a request to the SIA website.
/// Will retry the request with exponential backoff if it fails up to 3 times.
///
/// # Arguments
///
/// * `url` - The URL to make the request to.
/// * `payload` - The request payload.
///
/// # Returns
///
/// * `Result<Vec<LicenseState>, RequestError>` - A vector of license states if the search was successful, otherwise an error.
pub fn request_base(
    url: &str,
    payload: &Vec<(&str, &str)>,
) -> Result<Vec<LicenseState>, RequestError> {
    let mut backoff = 1;
    let client = Client::new();

    loop {
        let res = client.post(url).form(payload).send();

        match res {
            Ok(res) => {
                if res.status() == 200 {
                    let body = res.text().unwrap();
                    return match parse(&body) {
                        Ok(licenses) => Ok(licenses),
                        Err(err) => match err {
                            ParseError::NoLicensesFound => {
                                debug!("No licenses found.");
                                Ok(vec![])
                            }
                            ParseError::TooManySearchResults => {
                                debug!("Too many search results.");
                                Ok(vec![])
                            }
                            ParseError::NoLicenseContainersFound => {
                                warn!("No license containers found.");
                                Err(ParseFailed)
                            }
                        },
                    };
                } else {
                    error!("Request failed with status code: {}", res.status());
                    if backoff > 8 {
                        error!("Failed to make request after 3 attempts.");
                        return Err(RequestError::RequestFailed(
                            res.error_for_status().unwrap_err(),
                        ));
                    }
                }
            }
            Err(err) => {
                warn!("Error: {:?}", err);
                if backoff > 8 {
                    error!("Failed to make request after 3 attempts.");
                    return Err(RequestError::RequestFailed(err));
                }
            }
        }

        let delay = Duration::from_secs(backoff);
        std::thread::sleep(delay);
        backoff *= 2;
    }
}

/// Search for a license by license number.
///
/// # Arguments
///
/// * `payload` - The search payload.
///
/// # Returns
///
/// * `Result<Vec<LicenseState>, RequestError>` - A vector of license states if the search was successful, otherwise an error.
pub fn request_search_by_license(
    payload: SearchByLicense,
) -> Result<Vec<LicenseState>, RequestError> {
    let payload = payload.to_params();
    request_base(SEARCH_LICENSE_NUM_URL, &payload)
}

/// Search for a license by name.
///
/// # Arguments
///
/// * `payload` - The search payload.
///
/// # Returns
///
/// * `Result<Vec<LicenseState>, RequestError>` - A vector of license states if the search was successful, otherwise an error.
pub fn request_search_by_name(payload: SearchByName) -> Result<Vec<LicenseState>, RequestError> {
    let payload = payload.to_params();
    request_base(SEARCH_NAME_URL, &payload)
}

#[cfg(test)]
mod tests {
    use test_log;

    use super::*;

    #[test_log::test]
    fn test_request_search_by_license_fail() {
        let payload = SearchByLicense {
            license_no: "123456".to_string(),
        };

        let result = request_search_by_license(payload);
        assert!(result.is_ok());

        assert_eq!(result.unwrap().len(), 0);
    }

    #[test_log::test]
    fn test_request_search_by_license_success() {
        let known_license_no = std::env::var("KNOWN_LICENSE_NO");

        if known_license_no.is_err() {
            println!("Please set the environment variable KNOWN_LICENSE_NO to run this test.");
            return;
        }

        let payload = SearchByLicense {
            license_no: known_license_no.unwrap(),
        };

        let result = request_search_by_license(payload);

        assert!(result.is_ok());

        assert_eq!(result.unwrap().len(), 1);
    }

    #[test_log::test]
    fn test_request_search_by_name_success() {
        let known_first_name = std::env::var("KNOWN_FIRST_NAME");
        let known_last_name = std::env::var("KNOWN_LAST_NAME");

        if known_first_name.is_err() || known_last_name.is_err() {
            println!("Please set the environment variables KNOWN_FIRST_NAME and KNOWN_LAST_NAME to run this test.");
            return;
        }

        let payload = SearchByName {
            first_name: known_first_name.unwrap(),
            last_name: known_last_name.unwrap(),
            ..Default::default()
        };

        let result = request_search_by_name(payload);
        assert!(result.is_ok());

        assert_eq!(result.unwrap().len(), 2);
    }
}
