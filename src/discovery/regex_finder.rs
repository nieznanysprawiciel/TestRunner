
use super::finder::Finder;

use std::path::PathBuf;


pub struct RegexFinder
{

}



impl Finder for RegexFinder
{

	fn			discover			( &self, root_path: PathBuf )
	{
		
	}
}

impl RegexFinder
{
	pub fn		new					() -> RegexFinder
	{
		RegexFinder{}
	}
}
