use std::fmt::Display;

use chrono::{NaiveDate, TimeDelta};
use log::warn;
use serde::{Deserialize, Serialize};

/// Represents the state of a license.
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
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
        write!(f, "First Name: {} | Last Name: {} | License Number: {} | Role: {} | Sector: {} | Expiry: {} | Status: {} | Status Reason: {} | License Conditions: {}",
               self.first_name, self.last_name, self.license_number, self.role, self.sector, self.expiry, self.status, self.status_reason, self.license_conditions)
    }
}

/// Represents the role of a license.
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
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
            _ => write!(f, "Unknown Role"),
        }
    }
}

impl From<&String> for LicenseRole {
    fn from(s: &String) -> Self {
        let mut s = s.replace(|c: char| !c.is_alphanumeric(), "");
        s = s.to_lowercase();

        match s.as_str() {
            "frontline" => LicenseRole::Frontline,
            "nonfrontline" => LicenseRole::NonFrontline,
            _ => {
                warn!("Unknown role: {} - Please report this.", s);
                LicenseRole::Unknown
            }
        }
    }
}

/// Represents the sector of a license.
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
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
            _ => write!(f, "Unknown Sector"),
        }
    }
}

impl From<&String> for LicenseSector {
    fn from(s: &String) -> Self {
        let mut s = s.replace(|c: char| !c.is_alphanumeric(), "");
        s = s.to_lowercase();
        match s.as_str() {
            "cashandvaluablesintransit" => LicenseSector::CashInTransit,
            "cashintransit" => LicenseSector::CashInTransit,
            "closeprotection" => LicenseSector::CloseProtection,
            "doorsupervision" => LicenseSector::DoorSupervision,
            "publicspacesurveillancecctv" => LicenseSector::PublicSpaceSurveillance,
            "securityguard" => LicenseSector::SecurityGuard,
            "vehicleimmobilisation" => LicenseSector::VehicleImmobilisation,
            "keyholding" => LicenseSector::KeyHolding,
            "" => LicenseSector::NoSector,
            _ => {
                warn!("Unknown sector: {} - Please report this.", s);
                LicenseSector::Unknown
            }
        }
    }
}
