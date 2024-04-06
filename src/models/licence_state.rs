use chrono::{NaiveDate, TimeDelta};
use std::fmt::Display;
use log::warn;

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
    pub role: LicenseRole,
    /// The sector of this license
    pub sector: LicenseSector,
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

#[derive(Debug, Eq, PartialEq)]
pub enum LicenseRole {
    /// A license required for those who engage in licensable activities.
    Frontline,
    /// A license required for those who manage, supervise, or employ individuals who engage in licensable activities.
    NonFrontline,
    /// An unknown role - Used as a fallback.
    Unknown,
}

impl Display for LicenseRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LicenseRole::Frontline => write!(f, "Front Line"),
            LicenseRole::NonFrontline => write!(f, "Non Front Line"),
            _ => write!(f, "Unknown Role")
        }
    }
}

impl From<&String> for LicenseRole {
    fn from(s: &String) -> Self {
        match s.as_str() {
            "Front Line" => LicenseRole::Frontline,
            "Non Front Line" => LicenseRole::NonFrontline,
            _ => {
                warn!("Unknown role: {} - Please report this.", s);
                LicenseRole::Unknown
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum LicenseSector {
    /// Physical transportation of cash and valuables.
    CashInTransit,
    /// Protection of individuals.
    CloseProtection,
    /// Monitoring and controlling access to premises.
    DoorSupervision,
    /// Monitoring public spaces.
    PublicSpaceSurveillance,
    /// General security duties.
    SecurityGuard,
    /// Immobilisation of vehicles.
    VehicleImmobilisation,
    /// Holding keys to premises.
    KeyHolding,
    /// No specific sector.
    NoSector,
    /// An unknown sector - Used as a fallback.
    Unknown,
}

impl Display for LicenseSector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LicenseSector::CashInTransit => write!(f, "Cash and Valuables in Transit"),
            LicenseSector::CloseProtection => write!(f, "Close Protection"),
            LicenseSector::DoorSupervision => write!(f, "Door Supervision"),
            LicenseSector::PublicSpaceSurveillance => write!(f, "Public Space Surveillance (CCTV)"),
            LicenseSector::SecurityGuard => write!(f, "Security Guard"),
            LicenseSector::VehicleImmobilisation => write!(f, "Vehicle Immobilisation"),
            LicenseSector::KeyHolding => write!(f, "Key Holding"),
            LicenseSector::NoSector => write!(f, ""),
            _ => write!(f, "Unknown Sector")
        }
    }
}

impl From<&String> for LicenseSector {
    fn from(s: &String) -> Self {
        match s.as_str() {
            "Cash and Valuables in Transit" => LicenseSector::CashInTransit,
            "Close Protection" => LicenseSector::CloseProtection,
            "Door Supervision" => LicenseSector::DoorSupervision,
            "Public Space Surveillance (CCTV)" => LicenseSector::PublicSpaceSurveillance,
            "Security Guard" => LicenseSector::SecurityGuard,
            "Vehicle Immobilisation" => LicenseSector::VehicleImmobilisation,
            "Key Holding" => LicenseSector::KeyHolding,
            "" => LicenseSector::NoSector,
            _ => {
                warn!("Unknown sector: {} - Please report this.", s);
                LicenseSector::Unknown
            }
        }
    }
}