mod parse_selectors;
mod parsers;

use crate::models::payloads::{SearchByLicense, SearchByName};
use crate::{SEARCH_LICENSE_NUM_URL, SEARCH_NAME_URL};

use reqwest::Client;
use reqwest::Response;
use crate::models::LicenseState;

use crate::requests::parsers::parse;


/// Base function for making a request to the SIA API
///
/// # Arguments
///
/// * `res` - A Response object from the reqwest library.
///
/// # Returns
///
/// * `Option<Vec<LicenseState>>` - A vector of license states if the search was successful, otherwise None.
async fn request_base(res: Response) -> Option<Vec<LicenseState>> {
    if res.status() == 200 {
        let body = res.text().await.unwrap();
        let licenses = parse(&body);

        match licenses {
            Some(licenses) => {
                return Some(licenses);
            }
            None => {
                println!("No licenses found");
            }
        }
    }

    None
}


/// Search for a license by license number
///
/// # Arguments
///
/// * `payload` - A SearchByLicense object that contains the search parameters.
///
/// # Returns
///
/// * `Option<Vec<LicenseState>>` - A vector of license states if the search was successful, otherwise None.
pub async fn request_search_by_license(payload: SearchByLicense) -> Option<Vec<LicenseState>> {
    let client = Client::new();

    log::debug!("Searching for license number: {:?}" , payload);

    let res = client.post(SEARCH_LICENSE_NUM_URL).form(&payload).send().await;

    if res.is_ok() {
        let res = res.unwrap();
        request_base(res).await

    } else {
        println!("Error: {:?}", res.err());
        None
    }
}

/// Search for a license by name
///
/// # Arguments
///
/// * `payload` - A SearchByName object that contains the search parameters.
///
/// # Returns
///
/// * `Option<Vec<LicenseState>>` - A vector of license states if the search was successful, otherwise None.
pub async fn request_search_by_name(payload: SearchByName) -> Option<Vec<LicenseState>> {
    let client = Client::new();
    log::debug!("Searching for name: {:?}" , payload);

    let res = client.post(SEARCH_NAME_URL).form(&payload).send().await;

    if res.is_ok() {
        let res = res.unwrap();
        request_base(res).await

    } else {
        println!("Error: {:?}", res.err());
        None
    }
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
        assert_eq!(result, None);
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
        assert!(result.is_some());

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
        assert!(result.is_some());

        assert_eq!(result.unwrap().len(), 2);
    }
}