use std::fs::File;
use std::env;
use std::ffi::OsString;
use std::process;
use std::error::Error;
use serde::{Serialize, Deserialize};
use std::io;
use csv::ReaderBuilder;

#[derive(Debug, Serialize, Deserialize)]
//#[serde(rename_all = "PascalCase")]
struct Record {
    #[serde(deserialize_with = "csv::invalid_option")]
    id: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    name: Option<String>,
    #[serde(deserialize_with = "csv::invalid_option")]
    email: Option<String>
}

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let mut rdr = ReaderBuilder::new()
        .delimiter(b',')
        .comment(Some(b','))
        .comment(Some(b'.'))
        .from_path(file_path)?;
    for result in rdr.deserialize() {
        let record: Record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

//fn run() -> Result<(), Box<dyn Error>> {
    //let file_path = get_first_arg()?;
    //let mut rdr = csv::Reader::from_path(file_path)?;
    //for result in rdr.deserialize() {
        //let record: Record = result?;
        //println!("{:#?}", record.id);
    //}
    //Ok(())
//}

fn main()  {
    if let Err(err) = run() {
        println!("{:?}", err);
        process::exit(1);
    }
}
