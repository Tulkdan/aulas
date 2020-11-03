use std::io::Error;

mod util;

fn main() -> Result<(), Error> {
    let names_filename = String::from("names.csv");
    let names_file = util::HelperFile::open(names_filename)?;

    let pronome_filename = String::from("pronomes.csv");
    let pronome_file = util::HelperFile::open(pronome_filename)?;

    let mut users: Vec<util::User> = Vec::new();

    for name in names_file.data {
        let last_name = util::get_random_from_arr(&pronome_file.data);
        let user = util::User::new(name, last_name)?;
        users.push(user);
    }

    for user in users {
        println!("{:?}", user);
    }

    Ok(())
}
