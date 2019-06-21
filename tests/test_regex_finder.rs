use test_runner::discovery;
use test_runner::test_info;



use std::path::PathBuf;




#[test]
fn			test_discovery		()
{
	let discoverer = discovery::create_finder( "Regex" );
	let working_dir = PathBuf::from( "working-dir/test-structure1" );
	let tests_config = test_info::TestsConfig{ working_dir: working_dir.clone(), ..Default::default() };
	
	let to_execute = discoverer.discover( working_dir.clone(), &tests_config );

	assert_eq!( to_execute.len(), 3 );

	assert!( to_execute.contains( &test_info::TestDescription{ test_path: PathBuf::from( "working-dir/test-structure1/test_important_things.exe" ), working_dir: working_dir.clone() } ) );
	assert!( to_execute.contains( &test_info::TestDescription{ test_path: PathBuf::from( "working-dir/test-structure1/test_not_important_things.exe" ), working_dir: working_dir.clone() } ) );
	assert!( to_execute.contains( &test_info::TestDescription{ test_path: PathBuf::from( "working-dir/test-structure1/nested/test_important_things.exe" ), working_dir: working_dir.clone() } ) );
}

