
use crate::test_info::TestDescription;


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
		println!( "\nRunning test files:" );
		for test in &self.tests
		{
			println!( "{}", test.test_path.display() );
		}
	}
}

