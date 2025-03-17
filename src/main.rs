pub mod desustd;

use clap::{value_parser, Arg, Command};
fn main() {
    println!("Hello, world!");
    let app = Command::new("desu interpretier")
        .version("0.1.0")
        .author("F.F.")
        .about("an interpretier for 'desu'.")
        .arg(Arg::new("input file path").short("i").long("in").value_parser(value_parser!(str)))
        .arg(Arg::new("standard").short("s").long("std").value_parser(value_parser!(u32)))
        .get_matches();
}
