use clap::{Arg,  App};

fn main() {
    let _matches = App::new("echo")
        .version("0.1.0")
        .author("Patrick Buller")
        .about("Rust based version of echo")
        .arg(
            Arg::with_name("omit newline")
                .short("n")
                .help("Do not print with new line")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("input text")
                .required(true)
                .min_values(1),
        )
        .get_matches();

    
    println!("{:?}", std::env::args());
}
