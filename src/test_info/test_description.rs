
use std::path::PathBuf;


#[derive(PartialEq)]
pub struct TestDescription
{
	pub test_path: PathBuf,
	pub working_dir: PathBuf
}


