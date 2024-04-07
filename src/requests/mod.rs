mod parse_selectors;
mod parsers;
mod requests_async;

#[cfg(feature = "blocking")]
pub mod blocking;

pub use requests_async::{request_search_by_license, request_search_by_name};
