use chrono::{NaiveDate, TimeDelta};
use std::fmt::Display;

/// Represents the state of a license.
///
/// # Fields
///
/// * `first_name` - The first name of the license holder.
/// * `last_name` - The last name of the license holder.
/// * `license_number` - The license number.
/// * `role` - The role of the license holder.
/// * `sector` - The sector of the license holder.
/// * `expiry` - The expiry date of the license.
/// * `status` - The status of the license.
/// * `status_reason` - The reason for the status.
/// * `license_conditions` - The conditions of the license.
#[derive(Debug, Eq, PartialEq)]
pub struct LicenseState {
    pub first_name: String,
    pub last_name: String,
    pub license_number: String,
    pub role: String,
    pub sector: String,
    pub expiry: NaiveDate,
    pub status: String,
    pub status_reason: String,
    pub license_conditions: String,
}

impl LicenseState {
    /// Returns the number of days until the license expires.
    pub fn expires_in(&self) -> TimeDelta {
        self.expiry - chrono::Local::now().naive_local().date()
    }

    /// Returns the number of days remaining until the license expires.
    pub fn remaining_days(&self) -> i64 {
        self.expires_in().num_days()
    }
}

impl Display for LicenseState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        output.push_str(&format!("First Name: {} | ", self.first_name));
        output.push_str(&format!("Last Name: {} | ", self.last_name));
        output.push_str(&format!("License Number: {} | ", self.license_number));
        output.push_str(&format!("Role: {} | ", self.role));
        output.push_str(&format!("Sector: {} | ", self.sector));
        output.push_str(&format!("Expiry: {} | ", self.expiry));
        output.push_str(&format!("Status: {} | ", self.status));
        output.push_str(&format!("Status Reason: {} | ", self.status_reason));
        output.push_str(&format!(
            "License Conditions: {} | ",
            self.license_conditions
        ));
        write!(f, "{}", output)
    }
}
