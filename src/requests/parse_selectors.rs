use once_cell::sync::Lazy;
use scraper::Selector;

pub static CONTAINER_SELECTOR: Lazy<Selector> =
    Lazy::new(|| Selector::parse("div[class*='well']").unwrap());
pub static FIRST_NAME_SELECTOR: Lazy<Selector> =
    Lazy::new(|| Selector::parse("div:nth-of-type(1) > div:nth-of-type(1) > div > div").unwrap());
pub static LAST_NAME_SELECTOR: Lazy<Selector> =
    Lazy::new(|| Selector::parse("div:nth-of-type(1) > div:nth-of-type(2) > div > div").unwrap());
pub static LICENSE_NUMBER_SELECTOR: Lazy<Selector> =
    Lazy::new(|| Selector::parse("div:nth-of-type(2) > div:nth-of-type(1) > div > div").unwrap());
pub static ROLE_SELECTOR: Lazy<Selector> =
    Lazy::new(|| Selector::parse("div:nth-of-type(2) > div:nth-of-type(2) > div > div").unwrap());
pub static SECTOR_SELECTOR: Lazy<Selector> =
    Lazy::new(|| Selector::parse("div:nth-of-type(2) > div:nth-of-type(3) > div > div").unwrap());
pub static EXPIRY_SELECTOR: Lazy<Selector> =
    Lazy::new(|| Selector::parse("div:nth-of-type(3) > div:nth-of-type(1) > div > div").unwrap());
pub static STATUS_SELECTOR: Lazy<Selector> = Lazy::new(|| {
    Selector::parse("div:nth-of-type(3) > div:nth-of-type(2) > div > span:nth-of-type(1)").unwrap()
});
pub static STATUS_REASON_SELECTOR: Lazy<Selector> = Lazy::new(|| {
    Selector::parse("div:nth-of-type(4) > div > div:nth-of-type(2) > span:nth-of-type(1)").unwrap()
});

pub static LICENSE_CONDITIONS_SELECTOR: Lazy<Selector> =
    Lazy::new(|| Selector::parse("div:nth-of-type(5) > div > div:nth-of-type(2)").unwrap());
