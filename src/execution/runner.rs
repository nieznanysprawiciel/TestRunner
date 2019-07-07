
use crate::test_info::TestDescription;

use std::process::{ Command, Stdio };
use std::error::Error;
use std::io::prelude::*;


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

			let process = match Command::new( &test.test_path )
											.current_dir( &test.working_dir )
											.stdin( Stdio::piped() )
											.stdout( Stdio::piped() )
											.spawn()
			{
				Err( why ) => panic!( "couldn't spawn test: {}", why.description() ),
				Ok( process ) => process,
			};

			let mut test_output = String::new();
			match process.stdout.unwrap().read_to_string( &mut test_output )
			{
				Err( why ) => panic!( "couldn't read wc stdout: {}", why.description() ),
				Ok( _ ) => print!( "wc responded with:\n{}", test_output ),
			}

			println!( "{}", test_output );
		}
	}
}

