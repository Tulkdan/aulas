use rand::thread_rng;
use rand::seq::SliceRandom;
use std::fs::File;
use std::io::{BufReader, Error, BufRead, Write};
use std::path::Path;

#[derive(Debug)]
pub struct HelperFile {
    pub data: Vec<String>,
}

impl HelperFile {
    pub fn open(filename: &str) -> Result<HelperFile, Error> {
        let input = File::open(filename)?;
        let buffered = BufReader::new(input);

        let content: Vec<String> = buffered.lines()
            .into_iter()
            .filter_map(Result::ok)
            .collect();

        Ok(HelperFile { data: content })
    }

    pub fn write(filename: &str, data: &str) {
        let path = Path::new(filename);
        let display = path.display();

        // Open a file in write-only mode, returns `io::Result<File>`
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
        match file.write_all(data.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }
    }
}

#[derive(Debug)]
pub struct User<'a> {
    pub first_name: String,
    pub last_name: &'a str,
    pub email: String,
}

impl User<'_> {
    pub fn new<'a>(first_name: String, last_name: &'a str, provider: &str) -> Result<User<'a>, Error> {
        let email = format!("{}.{}@{}", first_name, last_name, provider);

        Ok(User { first_name, last_name, email })
    }

    pub fn format_to_csv(&self) -> String {
        format!("{};{};{}\n", self.first_name, self.last_name, self.email)
    }
}

pub fn get_random_from_arr<'a>(arr: &'a Vec<String>) -> &'a str {
    arr.choose(&mut thread_rng()).unwrap()
}

