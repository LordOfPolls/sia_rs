use serde::Serialize;

/// An object for searching the public register by name.
///
/// # Fields
///
/// * `last_name` - The last name of the person.
/// * `first_name` - The first name of the person.
/// * `middle_name` - The middle name of the person.
/// * `dob` - The date of birth of the person.
/// * `role` - The role of the person.
/// * `license_sector` - The license sector of the person.
#[derive(Serialize, Debug)]
pub struct SearchByName {
    // Used for https://services.sia.homeoffice.gov.uk/PublicRegister/SearchPublicRegisterBySurname
    #[serde(rename = "Surname")]
    pub last_name: String,
    #[serde(rename = "FirstName")]
    pub first_name: String,
    #[serde(rename = "MiddleName")]
    pub middle_name: String,
    #[serde(rename = "DateOfBirth")]
    pub dob: String,
    #[serde(rename = "Role")]
    pub role: String,
    #[serde(rename = "LicenseSector")]
    pub license_sector: String,
}

/// An object for searching the public register by license number.
///
/// # Fields
///
/// * `license_no` - The license number of the person.
#[derive(Serialize, Debug)]
pub struct SearchByLicense {
    // Used for https://services.sia.homeoffice.gov.uk/PublicRegister/SearchPublicRegisterByLicence
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
