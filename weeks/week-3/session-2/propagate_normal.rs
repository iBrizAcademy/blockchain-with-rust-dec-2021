use std::fs::File;
use std::io::Error;

fn open_file() -> Result<(), Error> {
    let file: Result<File, Error> = File::open("hello.txt");
    match file {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

fn main() {
    match open_file() {
        Ok(_) => println!("File is opened successfully!"),
        Err(e) => panic!(
            "Not able to open file. Here is the reason {:?}",
            e.to_string()
        ),
    }
}