use chrono::NaiveDate;
use log::{debug, warn};
use scraper::ElementRef;

use crate::errors::SIAError;
use crate::models::{LicenseRole, LicenseSector, LicenseState};
use crate::requests::parse_selectors::{
    CONTAINER_SELECTOR, EXPIRY_SELECTOR, FIRST_NAME_SELECTOR, LAST_NAME_SELECTOR,
    LICENSE_CONDITIONS_SELECTOR, LICENSE_NUMBER_SELECTOR, ROLE_SELECTOR, SECTOR_SELECTOR,
    STATUS_REASON_SELECTOR, STATUS_SELECTOR,
};

pub fn select_first(selector: &scraper::Selector, fragment: &scraper::Html) -> Option<String> {
    fragment
        .select(selector)
        .next()
        .map(|element| element.text().collect::<String>())
}

/// Post process a string to remove any leading or trailing whitespace and remove any trailing hyphens
///
/// # Arguments
///
/// * `input` - A string to post process
pub fn string_post_process(input: &str) -> String {
    let mut output = input.to_string();
    output = output.trim().to_string();
    if output.ends_with('-') {
        output.pop();
    }
    output = output.trim().to_owned();
    output
}

fn logged_unwrap_or<T: Default>(input: Option<T>, message: &str) -> T {
    match input {
        Some(value) => value,
        None => {
            warn!("{} - please report this issue", message);
            Default::default()
        }
    }
}

/// Parse the HTML body of the search results page
///
/// # Arguments
///
/// * `html_body` - The HTML body of the search results page
pub fn parse(html_body: &str) -> Result<Vec<LicenseState>, SIAError> {
    if html_body.contains("No results found") {
        return Err(SIAError::NoLicensesFound);
    }

    if html_body.contains("Too many search results") {
        return Err(SIAError::TooManyResults);
    }

    let document = scraper::Html::parse_document(html_body);
    let containers: Vec<ElementRef> = document.select(&CONTAINER_SELECTOR).collect();

    if containers.is_empty() {
        return Err(SIAError::NoLicensesFound);
    }

    debug!("Found {} license containers", containers.len());
    let mut licenses: Vec<LicenseState> = Vec::new();

    for container in containers {
        let fragment = scraper::Html::parse_fragment(&container.inner_html());

        let first_name = select_first(&FIRST_NAME_SELECTOR, &fragment);
        let last_name = select_first(&LAST_NAME_SELECTOR, &fragment);
        let license_number = select_first(&LICENSE_NUMBER_SELECTOR, &fragment);
        let role = select_first(&ROLE_SELECTOR, &fragment);
        let sector = select_first(&SECTOR_SELECTOR, &fragment);
        let expiry_raw = select_first(&EXPIRY_SELECTOR, &fragment);
        let status = select_first(&STATUS_SELECTOR, &fragment);
        let status_reason = select_first(&STATUS_REASON_SELECTOR, &fragment);
        let license_conditions = select_first(&LICENSE_CONDITIONS_SELECTOR, &fragment);

        let expiry: NaiveDate = if expiry_raw.is_some() {
            let expiry_t = logged_unwrap_or(expiry_raw, "Unable to find expiry date");
            let expiry_t = string_post_process(&expiry_t);
            NaiveDate::parse_from_str(&expiry_t, "%d %B %Y").unwrap_or_default()
        } else {
            NaiveDate::default()
        };

        if first_name.is_none()
            && last_name.is_none()
            && license_number.is_none()
            && role.is_none()
            && sector.is_none()
            && status.is_none()
            && status_reason.is_none()
            && license_conditions.is_none()
        {
            warn!("Unable to parse license - please report this issue. Aborting.");
            return Err(SIAError::ParseFailed);
        }

        let license = LicenseState {
            first_name: string_post_process(&logged_unwrap_or(
                first_name,
                "Unable to find first name",
            )),
            last_name: string_post_process(&logged_unwrap_or(
                last_name,
                "Unable to find last name",
            )),
            license_number: string_post_process(&logged_unwrap_or(
                license_number,
                "Unable to find license number",
            )),
            role: LicenseRole::from(&string_post_process(&logged_unwrap_or(
                role,
                "Unable to find role",
            ))),
            sector: LicenseSector::from(&string_post_process(&logged_unwrap_or(
                sector,
                "Unable to find sector",
            ))),
            expiry,
            status: string_post_process(&logged_unwrap_or(status, "Unable to find status")),
            status_reason: string_post_process(&logged_unwrap_or(
                status_reason,
                "Unable to find status reason",
            )),
            license_conditions: string_post_process(&logged_unwrap_or(
                license_conditions,
                "Unable to find license conditions",
            )),
        };

        debug!("Parsed license: {:?}", license.license_number);

        licenses.push(license);
    }

    Ok(licenses)
}
