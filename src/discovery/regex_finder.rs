extern crate walkdir;
extern crate regex;


use std::path::PathBuf;
use std::path::Path;

use super::finder::Finder;
use crate::test_info::TestDescription;
use crate::test_info::TestsConfig;

use regex::Regex;
use walkdir::WalkDir;
use std::env;
use std::fs;



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

		let current_dir = env::current_dir().unwrap();

		println!( "Current working directory: {}", current_dir.display() );
		println!( "Starting discovery in directory: {}", root_path.display() );

		for entry in WalkDir::new( root_path )
				.follow_links( true )
				.into_iter()
		{
			let path = entry.unwrap().into_path();
			if path.is_file()
			{
				println!( "Checking file: {}", path.display() );
				if self.is_test( &path, &config )
				{
					let test_description = TestDescription{ working_dir: config.working_dir.clone(), test_path: path };
					tests.push( test_description );
				}
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
		let test_regex = Regex::new( r"test.*.exe$" ).unwrap();

		test_regex.is_match( &filename )
	}

}


// ================================ //
// Tests

#[test]
fn			test_is_test									()
{
	let tests_config = TestsConfig{ working_dir: PathBuf::from( "" ), ..Default::default() };
	let finder = RegexFinder::new();

	assert!( finder.is_test( &PathBuf::from( "working-dir/structure1/test_important_things.exe" ), &tests_config ) );
	assert!( finder.is_test( &PathBuf::from( "working-dir/structure1/important_things_test.exe" ), &tests_config ) );
	assert!( finder.is_test( &PathBuf::from( "working-dir/structure1/Test_important_things_test.exe" ), &tests_config ) );
	assert!( finder.is_test( &PathBuf::from( "working-dir/structure1/Testimportant_things_test.exe" ), &tests_config ) );
}


#[test]
fn			test_is_test_directory_with_test_in_name		()
{
	let tests_config = TestsConfig{ working_dir: PathBuf::from( "" ), ..Default::default() };
	let finder = RegexFinder::new();

	assert_eq!( finder.is_test( &PathBuf::from( "working-dir/test-structure1/important_things.exe" ), &tests_config ), false );
	assert_eq!( finder.is_test( &PathBuf::from( "working-dir/structure1-test/important_things.exe" ), &tests_config ), false );
}


#[test]
fn			test_is_test_not_executables					()
{
	let tests_config = TestsConfig{ working_dir: PathBuf::from( "" ), ..Default::default() };
	let finder = RegexFinder::new();

	assert_eq!( finder.is_test( &PathBuf::from( "working-dir/test-structure1/important_things" ), &tests_config ), false );
	assert_eq!( finder.is_test( &PathBuf::from( "working-dir/test-structure1/important_things.png" ), &tests_config ), false );
}
