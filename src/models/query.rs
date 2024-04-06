use crate::models::{LicenseRole, LicenseSector};
use crate::models::payloads::{SearchByLicense, SearchByName};

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
/// let result = search(query);
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

    pub fn with_first_name(mut self, first_name: String) -> Self {
        self.first_name = Some(first_name);
        self
    }

    pub fn with_middle_name(mut self, middle_name: String) -> Self {
        self.middle_name = Some(middle_name);
        self
    }

    pub fn with_last_name(mut self, last_name: String) -> Self {
        self.last_name = Some(last_name);
        self
    }

    pub fn with_date_of_birth(mut self, date_of_birth: String) -> Self {
        self.date_of_birth = Some(date_of_birth);
        self
    }

    pub fn with_role(mut self, role: LicenseRole) -> Self {
        self.role = Some(role.to_string());
        self
    }

    pub fn with_license_sector(mut self, license_sector: LicenseSector) -> Self {
        self.license_sector = Some(license_sector.to_string());
        self
    }

    pub fn with_license_no(mut self, license_no: String) -> Self {
        self.license_no = Some(license_no);
        self
    }

    pub fn with_license_number(mut self, license_no: String) -> Self {
        self.license_no = Some(license_no);
        self
    }

    pub fn has_any(&self) -> bool {
        self.first_name.is_some()
            || self.middle_name.is_some()
            || self.last_name.is_some()
            || self.date_of_birth.is_some()
            || self.role.is_some()
            || self.license_sector.is_some()
            || self.license_no.is_some()
    }

    pub fn build(self) -> Self {
        self
    }

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

    pub fn to_search_by_license_payload(&self) -> SearchByLicense {
        SearchByLicense {
            license_no: self.license_no.clone().unwrap_or("".to_string()),
        }
    }
}
