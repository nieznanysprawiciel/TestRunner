
mod execution;
mod discovery;
mod test_info;


extern crate clap;


use clap::{ Arg, App };
use std::path::PathBuf;



struct Config
{
	pub discovery_dir: PathBuf
}


fn main()
{
	let config = parse_cmd_args();

	let discoverer = discovery::create_finder( "Regex" );
	
	let to_execute = discoverer.discover( config.discovery_dir );



	//let test_path = PathBuf::from( "C:\\dupa\\dupa" );
	//let working_dir = PathBuf::from( "C:\\dupa\\dupa\\work" );

	//let test_desc = test_info::TestDescription{ test_path, working_dir };

    //let to_execute = vec![ test_desc ];

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

	let discovery_dir = PathBuf::from( matches.value_of( "d" ).unwrap_or( "" ) );

	Config{ discovery_dir }
}
