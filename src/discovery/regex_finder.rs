extern crate walkdir;
extern crate regex;


use std::path::PathBuf;
use std::path::Path;

use super::finder::Finder;
use crate::test_info::TestDescription;
use crate::test_info::TestsConfig;

use regex::Regex;
use walkdir::WalkDir;



pub struct RegexFinder
{

}



impl Finder for RegexFinder
{

	// ================================ //
	//
	fn			discover			( &self, root_path: PathBuf, config: &TestsConfig ) -> Vec< TestDescription >
	{
		let mut tests = Vec::new();

		 for entry in WalkDir::new( root_path )
				.follow_links( true )
				.into_iter()
				.filter_entry( |entry| entry.file_type().is_file() )
		{
			let filepath = entry.unwrap().into_path();

			if self.is_test( &filepath, &config )
			{
				let test_description = TestDescription{ working_dir: config.working_dir.clone(), test_path: filepath };
				tests.push( test_description );
			}
		}

		return tests
	}
}

impl RegexFinder
{
	// ================================ //
	//
	pub fn		new					() -> RegexFinder
	{
		RegexFinder{}
	}

	// ================================ //
	//
	fn			is_test				( &self, path: &Path, config: &TestsConfig ) -> bool
	{
		let filename = path.file_name().unwrap().to_str().unwrap().to_lowercase();
		let test_regex = Regex::new( r"test.*$" ).unwrap();

		test_regex.is_match( &filename )
	}

}


// ================================ //
// Tests

#[test]
fn			test_is_test		()
{
	let tests_config = TestsConfig{ working_dir: PathBuf::from( "" ), ..Default::default() };
	let finder = RegexFinder::new();

	assert!( finder.is_test( &PathBuf::from( "working-dir/test-structure1/test_important_things.exe" ), &tests_config ) )
}

