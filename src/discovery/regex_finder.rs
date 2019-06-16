
use super::finder::Finder;
use crate::test_info::TestDescription;

use std::path::PathBuf;



pub struct RegexFinder
{

}



impl Finder for RegexFinder
{

	fn			discover			( &self, root_path: PathBuf ) -> Vec< TestDescription >
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
