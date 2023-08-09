extern crate clap;
extern crate csv;

use clap::{App, Arg};
use csv_reader::read_csv;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("CSV Reader")
        .version("1.0")
        .author("Emanuel One")
        .about("Reads CSV files")
        .arg(
            Arg::with_name("FILE")
                .help("Sets the input CSV file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    let filename = matches.value_of("FILE").unwrap();
    read_csv(filename)
}
