use rand::thread_rng;
use rand::seq::SliceRandom;
use std::fs::File;
use std::io::{BufReader, Error, BufRead, Write};
// use std::io::prelude::{write_all};
use std::path::Path;

#[derive(Debug)]
pub struct HelperFile {
    pub data: Vec<String>,
}

impl HelperFile {
    pub fn open(filename: String) -> Result<HelperFile, Error> {
        let path = format!("../{}", filename);

        let input = File::open(path)?;
        let buffered = BufReader::new(input);

        let mut content: Vec<String> = Vec::new();

        for line in buffered.lines() {
            content.push(String::from(line?));
        }

        Ok(HelperFile { data: content })
    }

    pub fn write(filename: &str, data: &String) {
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
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

impl User {
    pub fn new(first_name: String, last_name: String) -> Result<User, Error> {
        let providers = HelperFile::open(String::from("providers.csv"))?;
        let provider = get_random_from_arr(&providers.data);

        let email = format!("{}.{}@{}", first_name, last_name, provider);

        Ok(User { first_name, last_name, email })
    }

    pub fn format_to_csv(&self) -> String {
        format!("{};{};{}\n", self.first_name, self.last_name, self.email)
    }
}

pub fn get_random_from_arr(arr: &Vec<String>) -> String {
    arr.choose(&mut thread_rng()).unwrap().clone()
}

