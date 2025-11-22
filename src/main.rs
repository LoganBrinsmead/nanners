use std::fs::{self, File};
use std::io::{self, BufReader, prelude::*};
use std::str;
use walkdir::WalkDir;

fn main() -> io::Result<()> {
    // this is a good example to look into temporary lifetimes
    // TODO: not all paths are UTF-8 encoded
    let path = std::env::current_dir().unwrap().into_os_string();

    for entry in WalkDir::new(path) {
        // method path() belongs to the DirEntry inside of the Result class returned by WalkDir::new
        // have to use operators like expect(), ? or unwrap() to extract it
        // ? unpacks the Result returned from WalkDir if it is Ok()
        // Otherwise it returns an error
        let entry = entry?;
        if (!entry.path().is_dir() ) {
            let path = entry.path();
            let fileName = path.to_str();

            match fileName {
                Some(x) => {
                    let mut file = File::open(x)?;
                    let mut content = String::new();

                    file.read_to_string(&mut content)?;

                    if content.contains("nanners") {
                        println!("File that contains nanners: {}", x);
                    }

                    // println!("{}", content);
                }
                None => {
                    println!("File name not found.");
                }
            }
        }
    }

    Ok(())
}
