use std::io::Error;

mod util;

fn main() -> Result<(), Error> {
    let names_filename = "names.csv";
    let names_file = util::HelperFile::open(names_filename)?;

    let pronome_filename = "pronomes.csv";
    let pronome_file = util::HelperFile::open(pronome_filename)?;

    let provider_filename = "providers.csv";
    let providers = util::HelperFile::open(provider_filename)?;

    let mut users: Vec<util::User> = Vec::new();

    for name in names_file.data {
        let last_name = util::get_random_from_arr(&pronome_file.data);
        let provider = util::get_random_from_arr(&providers.data);
        let user = util::User::new(name, last_name, provider)?;
        users.push(user);
    }

    let mut output = String::from("");

    for user in users {
        output = format!("{}{}", output, user.format_to_csv());
    }

    util::HelperFile::write("output.csv", &output);

    Ok(())
}
