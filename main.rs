
use std::path::PathBuf;
use structopt::StructOpt;
use std::process;
use serde::{Serialize, Deserialize};
use std::error::Error;
use csv::{Reader, Writer};



#[derive(Debug, Serialize, Deserialize)]
struct Record {
    id: i32,
    name: String,
    email: String
}



#[derive(Debug, StructOpt)]
#[structopt(about = "Rust-csv parser")]
struct Opts {
    #[structopt(short, long, parse(from_os_str))]
    infile: PathBuf,
}



fn read() -> Result<(), Box<dyn Error>> {
    let opts = Opts::from_args();
    let mut rdr = Reader::from_path(opts.infile)?;
    for result in rdr.deserialize::<Record>() {
        match result {
            Ok(record) => println!("ID: {:#}, NAME: {:#}, EMAIL: {:#}", 
                                   record.id,
                                   record.name,
                                   record.email),
            Err(_e) => () //eprintln!("Error: {}", e)
        }
    }
    Ok(())
}




fn create() -> Result<(), Box<dyn Error>> {
    let opts = Opts::from_args();
    let mut wtr = Writer::from_path(opts.infile)?;
    wtr.serialize(Record {
        id: 501,
        name: "John Doe".to_string(),
        email: "john.doe@protonmail.com".to_string()
    })?;
    wtr.flush()?;
    Ok(())
}



fn main()  {
    if let Err(e) = read() {
        println!("{:?}", e);
        process::exit(1);
    }
    if let Err(e) = create() {
        println!("{:?}", e);
        process::exit(1);
    }
}
