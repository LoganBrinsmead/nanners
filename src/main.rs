use std::fs::{self, File};
use std::io::{self, BufReader, prelude::*};
use std::str;
use walkdir::WalkDir;

fn main() -> io::Result<()> {
    let path = ".";

    for entry in WalkDir::new(".") {
        let file = entry?.path().to_str();

        match file {
            Some(x) => {
                let reader = BufReader::new(x);
                for line in reader.lines() {
                    println!("{}", line?);
                }
            }
            None => {
                println!("File name not found.");
            }
        }

        let reader = BufReader::new(file);
        for line in reader.lines() {
            println!("{}", line?);
        }
        // println!("{}", entry?.path().display());
    }

    // for entry in fs::read_dir(path).unwrap() {
    //     let entry = entry.unwrap();
    //     // println!("{}", entry.path().display());
    //     let fileName =
    //     let file = File::open(String.from_utf8_lossy(entry))?;
    //     let reader = BufReader::new(file);
    // }

    // let file = File::open(name)?;
    // let reader = BufReader::new(file);

    // for line in reader.lines() {
    //     println!("{}", line?);
    // }

    Ok(())
}
