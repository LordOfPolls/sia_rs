pub use errors::RequestError;

pub use crate::models::payloads::{SearchByLicense, SearchByName};
pub use crate::models::{LicenseRole, LicenseSector, LicenseState, Query};
#[cfg(feature = "blocking")]
pub use crate::requests::blocking;

mod errors;
mod models;
mod requests;

pub const SEARCH_LICENSE_NUM_URL: &str =
    "https://services.sia.homeoffice.gov.uk/PublicRegister/SearchPublicRegisterByLicence";
pub const SEARCH_NAME_URL: &str =
    "https://services.sia.homeoffice.gov.uk/PublicRegister/SearchPublicRegisterBySurname";

/// Search for a license by either license number or name.
///
/// # Arguments
///
/// * `query` - A query object that contains the search parameters.
///
/// # Returns
///
/// * `Result<Vec<LicenseState>, RequestError>` - A vector of license states if the search was successful, otherwise an error.
pub async fn search(query: Query) -> Result<Vec<LicenseState>, RequestError> {
    if query.license_no.is_some() {
        let payload = query.to_search_by_license_payload();

        return requests::request_search_by_license(payload).await;
    }

    if query.has_any() {
        let payload = query.to_search_by_name_payload();

        return requests::request_search_by_name(payload).await;
    }

    Ok(Vec::new())
}

/// Search for a license by either license number or name synchronously.
///
/// # Arguments
///
/// * `query` - A query object that contains the search parameters.
///
/// # Returns
///
/// * `Result<Vec<LicenseState>, RequestError>` - A vector of license states if the search was successful, otherwise an error.
#[cfg(feature = "blocking")]
pub fn search_sync(query: Query) -> Result<Vec<LicenseState>, RequestError> {
    if query.license_no.is_some() {
        let payload = query.to_search_by_license_payload();

        return blocking::request_search_by_license(payload);
    }

    if query.has_any() {
        let payload = query.to_search_by_name_payload();

        return blocking::request_search_by_name(payload);
    }

    Ok(Vec::new())
}

#[cfg(test)]
mod tests {
    use crate::models::{LicenseRole, LicenseSector};

    use super::*;

    #[test_log::test]
    #[cfg(feature = "blocking")]
    fn test_search_with_name() {
        let query = Query::new()
            .with_last_name("Smith".to_string())
            .with_first_name("John".to_string())
            .with_middle_name("James".to_string())
            .with_date_of_birth("01/01/1970".to_string())
            .with_role(LicenseRole::Frontline)
            .with_license_sector(LicenseSector::DoorSupervision);

        let result = search_sync(query);

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);

        let known_first_name = std::env::var("KNOWN_FIRST_NAME");
        let known_last_name = std::env::var("KNOWN_LAST_NAME");

        if known_first_name.is_err() || known_last_name.is_err() {
            println!("Please set the environment variables KNOWN_FIRST_NAME and KNOWN_LAST_NAME to run this test.");
            return;
        }
        let query = Query::new()
            .with_first_name(known_first_name.unwrap())
            .with_last_name(known_last_name.unwrap());

        let result = search_sync(query);

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 2);
    }

    #[test_log::test]
    #[cfg(feature = "blocking")]
    fn test_search_with_license_no() {
        let query = Query::new().with_license_no("123456".to_string());

        let result = search_sync(query);

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);

        let known_license_no = std::env::var("KNOWN_LICENSE_NO");

        if known_license_no.is_err() {
            println!("Please set the environment variable KNOWN_LICENSE_NO to run this test.");
            return;
        }

        let query = Query::new().with_license_no(known_license_no.unwrap());
        let result = search_sync(query);

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);
    }

    #[test_log::test(tokio::test)]
    async fn test_search_with_name_async() {
        let query = Query::new()
            .with_last_name("Smith".to_string())
            .with_first_name("John".to_string())
            .with_middle_name("James".to_string())
            .with_date_of_birth("01/01/1970".to_string())
            .with_role(LicenseRole::Frontline)
            .with_license_sector(LicenseSector::DoorSupervision);

        let result = search(query).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);

        let known_first_name = std::env::var("KNOWN_FIRST_NAME");
        let known_last_name = std::env::var("KNOWN_LAST_NAME");

        if known_first_name.is_err() || known_last_name.is_err() {
            println!("Please set the environment variables KNOWN_FIRST_NAME and KNOWN_LAST_NAME to run this test.");
            return;
        }
        let query = Query::new()
            .with_first_name(known_first_name.unwrap())
            .with_last_name(known_last_name.unwrap());

        let result = search(query).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 2);
    }
}
