
use std::path::PathBuf;

use super::finder::Finder;
use crate::test_info::TestDescription;
use crate::test_info::TestsConfig;



pub struct RegexFinder
{

}



impl Finder for RegexFinder
{

	fn			discover			( &self, root_path: PathBuf, config: &TestsConfig ) -> Vec< TestDescription >
	{
		Vec::new()
	}
}

impl RegexFinder
{
	pub fn		new					() -> RegexFinder
	{
		RegexFinder{}
	}
}
