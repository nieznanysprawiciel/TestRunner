
use std::path::PathBuf;


pub struct TestsConfig
{
	pub working_dir: PathBuf,			// Global working directory, if we don't know where to run test.


	pub override_working_dir: bool,		// Use working_dir even if we know target directory.
}


impl Default for TestsConfig
{
    fn default() -> TestsConfig
	{
        TestsConfig
		{
			working_dir: PathBuf::from( "" ),
			override_working_dir: false,
        }
    }
}


