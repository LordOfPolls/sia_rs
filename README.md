# SIA_RS

A rust library for searching the UK's [Security Industry Authority (SIA) register of licenced security operatives](https://services.sia.homeoffice.gov.uk/PublicRegister/).

Providing an easy-to use interface for searching by licenses by licence number or details of the licence holder.

## Features
- Search for licenses by details (first name, last name, middle name, date of birth, role, and sector)
- Search for licenses by license number
- Returns all public information about the license holder
- Asynchronous and synchronous search functions
  - Synchronous functions are available with the `blocking` feature
- Full enum mapping for all possible roles and sectors

## Usage
```rust
use sia_rs::{Query, search};

#[tokio::main]
async fn main() {
    let query = Query::new()
        .with_last_name("Smith".to_string())
        .with_first_name("John".to_string());

    let results: Result<Vec<LicenseState>, RequestError> = search(&query).await;

    match results {
        Ok(licenses) => {
            for license in licenses {
                println!("{}", license);
            }
        },
        Err(e) => println!("Error: {}", e),
    }

    let query = Query::new()
        .with_license_number("1234567890123456".to_string());

    let results = search(&query).await;

    match results {
        Ok(licenses) => {
            for license in licenses {
                println!("{}", license);
            }
        },
        Err(e) => println!("Error: {}", e),
    }
}
```
All interactions are done through the `Query` struct, which is used to build the search query. 
The `search` function is used to perform the search and returns a `Vec<License>` containing the results.

### Blocking 
The `search_sync` function is a blocking function that will wait for the search to complete before returning the results.
This function is only available with the `blocking` feature enabled.

```toml
[dependencies]
sia_rs = { version = "*", features = ["blocking"] }
```


### Testing 
Some tests require real data and will only run if certain environment variables are set:
- `KNOWN_FIRST_NAME` - The first name of a known license holder
- `KNOWN_LAST_NAME` - The last name of a known license holder
- `KNOWN_LICENSE_NUMBER` - The license number of a known license holder

These are not included in the source code for privacy reasons.

## Notice
The SIA does not provide an official API for accessing the register, this library uses web scraping to retrieve the data. 
This means that the library may break if the SIA changes the structure of their website.

The author of this library is not responsible for any misuse of the data retrieved by this library.

## Contributing

Contributions are welcome, please open an issue or pull request.