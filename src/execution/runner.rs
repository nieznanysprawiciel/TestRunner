
use super::test_description::TestDescription;


pub struct Runner
{
	tests: Vec< TestDescription >
}


impl Runner
{
	// =========================== //
	//
	pub fn new			( tests: Vec< TestDescription > ) -> Runner
	{
		Runner{ tests }
	}

	// =========================== //
	//
	pub fn run			( &self )
	{
		for test in &self.tests
		{
			println!( "{}", test.test_path.display() );
		}
	}
}

