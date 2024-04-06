# SIA_RS

A rust library for searching the UK's [Security Industry Authority (SIA) register of licenced security operatives](https://services.sia.homeoffice.gov.uk/PublicRegister/).

Providing an easy-to use interface for searching by licenses by licence number or details of the licence holder.

## Features
- Search for licenses by name (first name, last name, middle name, date of birth, role, and sector)
- Search for licenses by license number
- Returns all public information about the license holder
- Asynchronous and synchronous search functions

## Usage
```rust
use sia_rs::{Query, search_sync};

fn main() {
    let query = Query::new()
        .with_last_name("Smith".to_string())
        .with_first_name("John".to_string());

    let results = search_sync(query);

    match results {
        Some(licenses) => {
            for license in licenses {
                println!("{}", license);
            }
        },
        None => println!("No licenses found"),
    }
    
    let query = Query::new()
        .with_license_number("1234567890123456".to_string());
    
    let results = search_sync(query);
    
    match results {
        Some(licenses) => {
            for license in licenses {
                println!("{}", license);
            }
        },
        None => println!("No licenses found"),
    }
}
```
All interactions are done through the `Query` struct, which is used to build the search query. 
The `search_sync` function is used to perform the search and returns a `Vec<License>` containing the results.
An async `search` function is also available, however, which follows the same pattern as the synchronous version.

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