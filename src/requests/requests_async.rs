// async variant of the request functions

use std::time::Duration;

use log::{debug, error, warn};
use reqwest::Client;

use crate::errors::SIAError;
use crate::models::payloads::{SearchByLicense, SearchByName};
use crate::models::LicenseState;
use crate::requests::parsers::parse;
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
async fn request_base(
    url: &str,
    payload: &Vec<(&str, &str)>,
) -> Result<Vec<LicenseState>, SIAError> {
    let mut backoff = 1;
    let client = Client::new();

    loop {
        let res = client.post(url).form(payload).send().await;

        match res {
            Ok(res) => {
                if res.status() == 200 {
                    let body = res.text().await.unwrap();
                    return match parse(&body) {
                        Ok(licenses) => Ok(licenses),
                        Err(err) => match err {
                            SIAError::NoLicensesFound => {
                                debug!("No licenses found.");
                                Ok(vec![])
                            }
                            SIAError::TooManyResults => {
                                debug!("Too many search results.");
                                Ok(vec![])
                            }
                            _ => {
                                error!("Failed to parse license data.");
                                Err(SIAError::Error(
                                    "Failed to parse license data - ".to_owned()
                                        + err.to_string().as_str(),
                                ))
                            }
                        },
                    };
                } else {
                    error!("Request failed with status code: {}", res.status());
                    if backoff > 8 {
                        error!("Failed to make request after 3 attempts.");
                        return Err(SIAError::RequestFailed(res.error_for_status().unwrap_err()));
                    }
                }
            }
            Err(err) => {
                warn!("Error: {:?}", err);
                if backoff > 8 {
                    error!("Failed to make request after 3 attempts.");
                    return Err(SIAError::RequestFailed(err));
                }
            }
        }

        let delay = Duration::from_secs(backoff);
        tokio::time::sleep(delay).await;
        backoff *= 2;
    }
}

/// Search for a license by license number
///
/// # Arguments
///
/// * `payload` - A SearchByLicense object that contains the search parameters.
///
/// # Returns
///
/// * `Result<Vec<LicenseState>, RequestError>` - A vector of license states if the search was successful, otherwise an error.
pub async fn request_search_by_license(
    payload: SearchByLicense,
) -> Result<Vec<LicenseState>, SIAError> {
    log::debug!("Searching for license number: {:?}", payload);
    request_base(SEARCH_LICENSE_NUM_URL, &payload.to_params()).await
}

/// Search for a license by name
///
/// # Arguments
///
/// * `payload` - A SearchByName object that contains the search parameters.
///
/// # Returns
///
/// * `Result<Vec<LicenseState>, RequestError>` - A vector of license states if the search was successful, otherwise an error.
pub async fn request_search_by_name(payload: SearchByName) -> Result<Vec<LicenseState>, SIAError> {
    log::debug!("Searching for name: {:?}", payload);
    request_base(SEARCH_NAME_URL, &payload.to_params()).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test(tokio::test)]
    async fn test_request_search_by_license_fail() {
        let payload = SearchByLicense {
            license_no: "123456".to_string(),
        };

        let result = request_search_by_license(payload).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[test_log::test(tokio::test)]
    async fn test_request_search_by_license_success() {
        let known_license_no = std::env::var("KNOWN_LICENSE_NO");

        if known_license_no.is_err() {
            println!("Please set the environment variable KNOWN_LICENSE_NO to run this test.");
            return;
        }

        let payload = SearchByLicense {
            license_no: known_license_no.unwrap(),
        };

        let result = request_search_by_license(payload).await;
        assert!(result.is_ok());

        assert_eq!(result.unwrap().len(), 1);
    }

    #[test_log::test(tokio::test)]
    async fn test_request_search_by_name_success() {
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

        let result = request_search_by_name(payload).await;
        assert!(result.is_ok());

        assert_eq!(result.unwrap().len(), 2);
    }
}
