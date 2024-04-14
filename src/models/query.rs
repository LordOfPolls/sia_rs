use crate::models::payloads::{SearchByLicense, SearchByName};
use crate::models::{LicenseRole, LicenseSector};
use crate::{LicenseState, RequestError};

/// A query object that contains the search parameters.
/// Follows the builder pattern.
///
/// # Example
///
/// ```
/// use sia_rs::{Query, search};
/// let query = Query::new()
///    .with_last_name("Smith".to_string())
///   .with_first_name("John".to_string());
///
/// let result = search(&query);
/// ```
#[derive(Default, Debug, Clone)]
pub struct Query {
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub date_of_birth: Option<String>,
    pub role: Option<String>,
    pub license_sector: Option<String>,
    pub license_no: Option<String>,
}

impl Query {
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the first name of the license holder.
    pub fn with_first_name(mut self, first_name: String) -> Self {
        self.first_name = Some(first_name);
        self
    }

    /// Sets the middle name of the license holder.
    pub fn with_middle_name(mut self, middle_name: String) -> Self {
        self.middle_name = Some(middle_name);
        self
    }

    /// Sets the last name of the license holder.
    pub fn with_last_name(mut self, last_name: String) -> Self {
        self.last_name = Some(last_name);
        self
    }

    /// Sets the date of birth of the license holder.
    pub fn with_date_of_birth(mut self, date_of_birth: String) -> Self {
        self.date_of_birth = Some(date_of_birth);
        self
    }

    /// Sets the role of the license.
    pub fn with_role(mut self, role: LicenseRole) -> Self {
        self.role = Some(role.to_string());
        self
    }

    /// Sets the sector of the license.
    pub fn with_license_sector(mut self, license_sector: LicenseSector) -> Self {
        self.license_sector = Some(license_sector.to_string());
        self
    }

    /// Sets the license number.
    pub fn with_license_no(mut self, license_no: String) -> Self {
        self.license_no = Some(license_no);
        self
    }

    /// Checks if any search parameters are set.
    pub fn with_license_number(mut self, license_no: String) -> Self {
        self.license_no = Some(license_no);
        self
    }

    /// Checks if any search parameters are set.
    pub fn has_any(&self) -> bool {
        self.first_name.is_some()
            || self.middle_name.is_some()
            || self.last_name.is_some()
            || self.date_of_birth.is_some()
            || self.role.is_some()
            || self.license_sector.is_some()
            || self.license_no.is_some()
    }

    /// Builds the query object.
    pub fn build(self) -> Self {
        self
    }

    /// Converts the query object to a SearchByName payload.
    pub fn to_search_by_name_payload(&self) -> SearchByName {
        SearchByName {
            last_name: self.last_name.clone().unwrap_or("".to_string()),
            first_name: self.first_name.clone().unwrap_or("".to_string()),
            middle_name: self.middle_name.clone().unwrap_or("".to_string()),
            dob: self.date_of_birth.clone().unwrap_or("".to_string()),
            role: self.role.clone().unwrap_or("".to_string()),
            license_sector: self.license_sector.clone().unwrap_or("".to_string()),
        }
    }

    /// Converts the query object to a SearchByLicense payload.
    pub fn to_search_by_license_payload(&self) -> SearchByLicense {
        SearchByLicense {
            license_no: self.license_no.clone().unwrap_or("".to_string()),
        }
    }

    /// Alias function for crate::search
    ///
    /// Takes no arguments and returns a Result<Vec<LicenseState>, RequestError>
    pub async fn search(&self) -> Result<Vec<LicenseState>, RequestError> {
        crate::search(self).await
    }

    /// Alias function for crate::search_sync
    ///
    /// Takes no arguments and returns a Result<Vec<LicenseState>, RequestError>
    #[cfg(feature = "blocking")]
    pub fn search_sync(&self) -> Result<Vec<LicenseState>, RequestError> {
        crate::search_sync(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::models::{LicenseRole, LicenseSector};

    use super::*;

    #[test_log::test]
    #[cfg(feature = "blocking")]
    fn test_query_search_with_name() {
        let result = Query::new()
            .with_last_name("Smith".to_string())
            .with_first_name("John".to_string())
            .with_middle_name("James".to_string())
            .with_date_of_birth("01/01/1970".to_string())
            .with_role(LicenseRole::Frontline)
            .with_license_sector(LicenseSector::DoorSupervision)
            .search_sync();

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);

        let known_first_name = std::env::var("KNOWN_FIRST_NAME");
        let known_last_name = std::env::var("KNOWN_LAST_NAME");

        if known_first_name.is_err() || known_last_name.is_err() {
            println!("Please set the environment variables KNOWN_FIRST_NAME and KNOWN_LAST_NAME to run this test.");
            return;
        }

        let result = Query::new()
            .with_first_name(known_first_name.unwrap())
            .with_last_name(known_last_name.unwrap())
            .search_sync();

        assert!(result.is_ok());
        assert!(result.unwrap().len() > 0);
    }

    #[test_log::test]
    #[cfg(feature = "blocking")]
    fn test_query_search_with_license_no() {
        let query = Query::new().with_license_no("123456".to_string());

        assert_eq!(query.license_no, Some("123456".to_string()));

        let known_license_no = std::env::var("KNOWN_LICENSE_NO");

        if known_license_no.is_err() {
            println!("Please set the environment variable KNOWN_LICENSE_NO to run this test.");
            return;
        }

        let result = Query::new()
            .with_license_no(known_license_no.unwrap())
            .search_sync();

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);
    }

    #[test_log::test(tokio::test)]
    async fn test_query_search_with_name_async() {
        let result = Query::new()
            .with_last_name("Smith".to_string())
            .with_first_name("John".to_string())
            .with_middle_name("James".to_string())
            .with_date_of_birth("01/01/1970".to_string())
            .with_role(LicenseRole::Frontline)
            .with_license_sector(LicenseSector::DoorSupervision)
            .search()
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);

        let known_first_name = std::env::var("KNOWN_FIRST_NAME");
        let known_last_name = std::env::var("KNOWN_LAST_NAME");

        if known_first_name.is_err() || known_last_name.is_err() {
            println!("Please set the environment variables KNOWN_FIRST_NAME and KNOWN_LAST_NAME to run this test.");
            return;
        }

        let result = Query::new()
            .with_first_name(known_first_name.unwrap())
            .with_last_name(known_last_name.unwrap())
            .search()
            .await;

        assert!(result.is_ok());
        assert!(result.unwrap().len() > 0);
    }

    #[test_log::test(tokio::test)]
    async fn test_query_search_with_license_no_async() {
        let query = Query::new()
            .with_license_no("123456".to_string())
            .search()
            .await;

        assert!(query.is_ok());
        assert_eq!(query.unwrap().len(), 0);

        let known_license_no = std::env::var("KNOWN_LICENSE_NO");

        if known_license_no.is_err() {
            println!("Please set the environment variable KNOWN_LICENSE_NO to run this test.");
            return;
        }

        let query = Query::new()
            .with_license_no(known_license_no.unwrap())
            .search()
            .await;

        assert!(query.is_ok());
        assert_eq!(query.unwrap().len(), 1);
    }
}
