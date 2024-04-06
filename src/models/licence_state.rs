use chrono::{NaiveDate, TimeDelta};
use std::fmt::Display;

/// Represents the state of a license.
#[derive(Debug, Eq, PartialEq)]
pub struct LicenseState {
    /// The first name of the license holder.
    pub first_name: String,
    /// The last name of the license holder.
    pub last_name: String,
    /// The license number.
    pub license_number: String,
    /// The role of this license
    pub role: String,
    /// The sector of this license
    pub sector: String,
    /// The expiry date of the license.
    pub expiry: NaiveDate,
    /// The status of the license.
    pub status: String,
    /// The reason for the status.
    pub status_reason: String,
    /// The conditions of the license.
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
