use reqwest;
use semver::Version;

/// The comparitive status of the version query.
#[derive(Debug, PartialEq)]
pub enum Status {
	/// The version is behind the one on `crates.io`.
	Behind,
	/// The version is equal to the one on `crates.io`.
	Equal,
	/// The version is ahead of the one on `crates.io`.
	Ahead,
}

/// Errors in requesting or parsing the query.
pub enum Error {
	/// Failed to parse the response for a max version of the crate.
	ParseError,
}

/// Get the `crates.io` version of the specified crate.
pub fn get(crate_name: &str) -> Result<Version, Error> {
	Version::parse(parse(web_req(crate_name)?))
}

/// Gets the `crates.io` version of the specified crate and compares it to the specified version.
pub fn query(crate_name: &str, version: &str) -> Result<Status, ()> {
	get(crate_name)?
}

fn parse(text: &str) -> Result<&str, Error> {
	match text.split('\"').skip_while(|&x| x != "max_version").nth(2) {
		// json format ("max_version":"#.#.#") hence will parse as [max_version, :, #,#,#]
		Some(ver) => Ok(ver),
		None => Err(Error::ParseError),
	}
}

fn web_req(crate_name: &str) -> Result<String, ()> {
	match reqwest::get(&format!("https://crates.io/api/v1/crates/{}", crate_name)) {
		Ok(mut response) => match response.text() {
			Ok(text) => Ok(text),
			Err(_) => Err(()),
		},
		Err(_) => Err(()),
	}
}

#[test]
fn parse_test() {
	assert_eq!(parse(r#""max_version":"0.4.2""#, Some("0.4.2")));
	assert_eq!(parse(r#""max_version":"0..2""#, Some("0..2")));
	assert_eq!(parse(r#""max_version0.4.2""#, None));
}

#[test]
fn test_web_req() {
	// verify that the return crate is the right one!
	let req = web_req("papyrus");
	match req {
		Err(_) => panic!("failed to query crates.io"),
		Ok(text) => {
			assert!(text.starts_with(r#"{"crate":{"id":"papyrus","name":"papyrus","#));
		}
	}
}
