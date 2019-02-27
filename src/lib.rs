//! [![Build Status](https://travis-ci.com/kurtlawrence/cratesiover.svg?branch=master)](https://travis-ci.com/kurtlawrence/cratesiover)
//! [![Latest Version](https://img.shields.io/crates/v/cratesiover.svg)](https://crates.io/crates/cratesiover)
//! [![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/cratesiover)
//! [![codecov](https://codecov.io/gh/kurtlawrence/cratesiover/branch/master/graph/badge.svg)](https://codecov.io/gh/kurtlawrence/cratesiover)
//!
//! Query and compare the semver of a crate on crates.io.
//!
//! See the [rs docs](https://docs.rs/cratesiover/). [Github repo.](https://github.com/kurtlawrence/cratesiover)
//!
//! # Example
//!
//! ```rust
//! use cratesiover::Status;
//! let query = cratesiover::query("cratesiover", &env!("CARGO_PKG_VERSION")).unwrap();
//!
//! match query {
//!   Status::Behind(ver) => println!("crate is behind the version on crates.io {}", ver),
//!   Status::Equal(ver) => println!("crate is equal to the version on crates.io {}", ver),
//!   Status::Ahead(ver) => println!("crate is ahead of the version on crates.io {}", ver),
//! }
//! ```
#![warn(missing_docs)]

use colored::*;
use linefeed::Terminal;
use reqwest;
use semver::Version;
use std::cmp::Ordering;
use std::io::{self, Write};

/// The comparitive status of the version query.
/// Each variant contains the `crates.io` version number.
#[derive(Debug, PartialEq)]
pub enum Status {
	/// The version is behind the one on `crates.io`.
	Behind(Version),
	/// The version is equal to the one on `crates.io`.
	Equal(Version),
	/// The version is ahead of the one on `crates.io`.
	Ahead(Version),
}

/// Errors in requesting or parsing the query.
#[derive(Debug)]
pub enum Error {
	/// Failed to parse the response for a max version of the crate.
	ParseError,
	/// Failed to parse the reponse into a `semver::Version`.
	SemVerError(semver::SemVerError),
	/// Failed to successfully make a request to or receive a response from `crates.io`.
	RequestError(reqwest::Error),
}

struct Writer<'a, T: Terminal>(&'a T);

impl<'a, T: Terminal> Writer<'a, T> {
	pub fn overwrite_current_console_line(&self, line: &str) -> io::Result<()> {
		let mut wtr = self.0.lock_write();
		wtr.move_to_first_column()?;
		wtr.clear_to_screen_end()?;
		wtr.write(line)
	}
}

impl<'a, T: Terminal> Write for Writer<'a, T> {
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		let mut wtr = self.0.lock_write();
		wtr.write(&String::from_utf8_lossy(buf)).unwrap();
		Ok(buf.len())
	}

	fn flush(&mut self) -> io::Result<()> {
		Ok(())
	}
}

/// Get the `crates.io` version of the specified crate.
pub fn get(crate_name: &str) -> Result<Version, Error> {
	Version::parse(parse(&web_req(crate_name)?)?).map_err(|e| Error::SemVerError(e))
}

/// Gets the `crates.io` version of the specified crate and compares it to the specified version.
///
/// # Example
/// ```rust
/// use cratesiover::{ query, Status };
/// let query = query("cratesiover", "0.1.0").unwrap();
/// match query {
///  Status::Behind(ver) => println!("crate is behind the version on crates.io {}", ver),
///  Status::Equal(ver) => println!("crate is equal to the version on crates.io {}", ver),
///  Status::Ahead(ver) => println!("crate is ahead of the version on crates.io {}", ver),
/// }
/// ```
pub fn query(crate_name: &str, version: &str) -> Result<Status, Error> {
	let version = Version::parse(version).map_err(|e| Error::SemVerError(e))?;
	Ok(cmp(&version, get(crate_name)?))
}

/// Query and compare the crate version number. Write to stdout the status.
pub fn output(crate_name: &str, version: &str) -> io::Result<()> {
	Ok(output_with_term(
		crate_name,
		version,
		&linefeed::DefaultTerminal::new()?,
	))
}

/// Query and compare the crate version number. Write to the given terminal the status.
pub fn output_with_term<Term: Terminal>(crate_name: &str, version: &str, terminal: &Term) {
	print!("{}", "Checking for later version...".bright_yellow());
	io::stdout().flush().is_ok();
	let print_line = match query(crate_name, version) {
		Ok(status) => match status {
			Status::Equal(ver) => format!(
				"{}{}",
				"Running the latest papyrus version ".bright_green(),
				ver.to_string().bright_green()
			),
			Status::Behind(ver) => format!(
				"{}",
				format!(
					"The current papyrus version {} is old, please update to {}",
					version, ver
				)
				.bright_red()
			),
			Status::Ahead(ver) => format!(
				"{}",
				format!(
					"The current papyrus version {} is ahead of the crates.io version {}",
					version, ver
				)
				.bright_purple()
			),
		},
		Err(_) => format!("{}", "Failed to query crates.io".bright_yellow()),
	};
	let mut wtr = Writer(terminal);
	wtr.overwrite_current_console_line(&print_line).unwrap();
	writeln!(wtr, "",).unwrap();
}

fn parse(text: &str) -> Result<&str, Error> {
	match text.split('\"').skip_while(|&x| x != "max_version").nth(2) {
		// json format ("max_version":"#.#.#") hence will parse as [max_version, :, #,#,#]
		Some(ver) => Ok(ver),
		None => Err(Error::ParseError),
	}
}

fn web_req(crate_name: &str) -> Result<String, Error> {
	reqwest::get(&format!("https://crates.io/api/v1/crates/{}", crate_name))
		.map_err(|e| Error::RequestError(e))?
		.text()
		.map_err(|e| Error::RequestError(e))
}

fn cmp(current: &Version, cratesio: Version) -> Status {
	match current.cmp(&cratesio) {
		Ordering::Less => Status::Behind(cratesio),
		Ordering::Equal => Status::Equal(cratesio),
		Ordering::Greater => Status::Ahead(cratesio),
	}
}

#[test]
fn parse_test() {
	assert_eq!(parse(r#""max_version":"0.4.2""#).unwrap(), "0.4.2");
	assert_eq!(parse(r#""max_version":"0..2""#).unwrap(), "0..2");
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

#[test]
fn cmp_test() {
	let one_pt_oh = Version::parse("1.0.0").unwrap();
	let pt_one_oh = Version::parse("0.1.0").unwrap();
	assert_eq!(
		cmp(&one_pt_oh, one_pt_oh.clone(),),
		Status::Equal(one_pt_oh.clone())
	);
	assert_eq!(
		cmp(&pt_one_oh, one_pt_oh.clone(),),
		Status::Behind(one_pt_oh.clone())
	);
	assert_eq!(
		cmp(&one_pt_oh, pt_one_oh.clone()),
		Status::Ahead(pt_one_oh.clone())
	);
}
