use std::env;
use std::process;

use minigrep::Config;

fn main() {
    /*
        There's a few things going on here with args.
        We instantiate args and specify the type as
        a Vector which contains Strings. Then we use
        env::args() to specify that we want to collect
        command line args and use .collect() to turn
        those args into the collection we'll be using.
    */
    let args: Vec<String> = env::args().collect();

    // Here we're destructuring the tuple to extract query and filename
    // by calling parse_config and passing it a reference to the args vector.
    let config: minigrep::Config = Config::new(&args).unwrap_or_else(|err: &str| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for: {:?}", config.query);
    println!("In file: {:?}", config.filename);

    if let Err(e) = minigrep::run(config) {
        // When printing errors if we wanted to print to the error stream, we add the char 'e'
        // before println! -> eprintln!();
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }
}
