use std::env;
use std::error::Error;
use std::io;
use std::process;
use std::io::stdout;
use std::io::Write;

fn run() -> Result<(), Box<dyn Error>> {
    let query = match env::args().nth(1) {
        None => return Err(From::from("expected 1 argument, but got none")),
        Some(query) => query,
    };
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut stdout = stdout();
    let headers = rdr.headers()?;
    let field_index = headers.iter().enumerate().find(|(i, o)| o == &query).unwrap().0;
    for result in rdr.records() {
        let record = result?;
        stdout.write_all(record.get(field_index).unwrap().as_bytes());
        stdout.write_all(&[b'\n']);
    }
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}