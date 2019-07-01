
mod execution;
mod discovery;
mod test_info;


extern crate clap;


use clap::{ Arg, App };
use std::path::PathBuf;
use std::env;



struct Config
{
	pub discovery_dir: PathBuf
}


fn main()
{
	let config = parse_cmd_args();

	let discoverer = discovery::create_finder( "Regex" );
	let tests_config = test_info::TestsConfig{ working_dir: config.discovery_dir.clone(), ..Default::default() };
	
	let to_execute = discoverer.discover( config.discovery_dir.clone(), &tests_config );

	let runner = execution::Runner::new( to_execute );

	runner.run();
}

fn parse_cmd_args() -> Config
{
    let matches = App::new( "Test Runner" )
        .version( "0.1.0" )
        .author( "nieznanysprawiciel <nieznany.sprawiciel@gmail.com>" )
        .about( "Application for discovering and running tests in visual studio projects without using build in testing tools." )
        .arg( Arg::with_name( "dir" )
            .short( "d" )
            .long( "discovery dir" )
            .help( "Directory to look for tests." ))
        .get_matches();

	let current_dir = env::current_dir().unwrap();
	let discovery_dir = PathBuf::from( matches.value_of( "d" ).unwrap_or( current_dir.to_str().unwrap() ) );

	Config{ discovery_dir }
}
