use serde::Serialize;

/// An object for searching the public register by name.
/// Used for https://services.sia.homeoffice.gov.uk/PublicRegister/SearchPublicRegisterBySurname
#[derive(Serialize, Debug)]
pub struct SearchByName {
    /// Their last name
    #[serde(rename = "Surname")]
    pub last_name: String,
    /// Their first name
    #[serde(rename = "FirstName")]
    pub first_name: String,
    /// Their middle name
    #[serde(rename = "MiddleName")]
    pub middle_name: String,
    #[serde(rename = "DateOfBirth")]
    /// Their date of birth
    pub dob: String,
    #[serde(rename = "Role")]
    /// Their sia licence role
    pub role: String,
    /// Their sia licence sector
    #[serde(rename = "LicenseSector")]
    pub license_sector: String,
}

/// An object for searching the public register by license number.
/// Used for https://services.sia.homeoffice.gov.uk/PublicRegister/SearchPublicRegisterByLicence
#[derive(Serialize, Debug)]
pub struct SearchByLicense {
    /// The license number to search for
    #[serde(rename = "LicenseNo")]
    pub license_no: String,
}

impl Default for SearchByName {
    fn default() -> Self {
        Self {
            last_name: "".to_string(),
            first_name: "".to_string(),
            middle_name: "".to_string(),
            dob: "".to_string(),
            role: "".to_string(),
            license_sector: "".to_string(),
        }
    }
}
