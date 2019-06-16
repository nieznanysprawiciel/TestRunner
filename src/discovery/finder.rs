
use std::path::PathBuf;


use super::regex_finder::RegexFinder;


pub trait Finder
{
	fn			discover			( &self, root_path: PathBuf );
}


pub fn			create_finder		( implementation: &str ) -> Box< dyn Finder >
{
	match implementation
	{
		"Regex" => Box::new( RegexFinder::new() ),
		
		// Return RegexFinder by default, because I don't know what to return.
		_ => Box::new( RegexFinder::new() )
	}
}

