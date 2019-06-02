
mod execution;



use std::path::PathBuf;



fn main()
{
	let test_path = PathBuf::from( "C:\\dupa\\dupa" );
	let working_dir = PathBuf::from( "C:\\dupa\\dupa\\work" );

	let test_desc =  execution::TestDescription{ test_path, working_dir };

    let to_execute = vec![ test_desc ];

	let runner = execution::Runner::new( to_execute );

	runner.run();
}
