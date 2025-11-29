use std::fs::{File};
use std::io::{self, BufReader, prelude::*};
use walkdir::WalkDir;

fn main() -> io::Result<()> {
    // this is a good example to look into temporary lifetimes
    // TODO: not all paths are UTF-8 encoded
    let path = std::env::current_dir().unwrap();

    for entry in WalkDir::new(path) {
        // method path() belongs to the DirEntry inside of the Result class returned by WalkDir::new
        // have to use operators like expect(), ? or unwrap() to extract it
        // ? unpacks the Result returned from WalkDir if it is Ok()
        // Otherwise it returns an error
        let entry = entry?;
        if !entry.path().is_dir()  {
            let path = entry.path();
            let file_name = path.to_str();


            match file_name {
                Some(x) => {
                    let file = File::open(x)?;
                    let reader = BufReader::new(file);
                    let _content = String::new();
                    let mut line_count = 0;

                    for line in reader.lines() {
                        line_count += 1;

                        /* 
                            Result types need to be handled
                            account for situations where the result is OK, and for results where there is an error.
                            Just using ? is likely to produce serious errors in your application down the line.
                        
                         */
                        // TODO: better handling for invalid UTF-8 lines or handle UTF-8
                        let valid_line = match line {
                            Ok(e) => e,
                            Err(_) => {
                                println!("Warning: line did not contain valid UTF-8.");
                                continue;
                            }
                        };

                        if valid_line.contains("nanners") {
                            println!("NANNERS!\nFile that contains Nanners: {}, line number: {}", x, line_count);
                        }
                    }

                }
                None => {
                    println!("Error processing that filename.");
                    continue;
                }
            }
        }
    }

    Ok(())
}
