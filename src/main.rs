use clap::App;

fn main() {
    let _matches = App::new("echo")
        .version("0.1.0")
        .author("Patrick Buller")
        .about("Rust based version of echo")
        .get_matches();

    
    println!("{:?}", std::env::args());
}
