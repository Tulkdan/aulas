use std::io::Error;
use libc::{c_char, c_void};
use std::ptr::{null, null_mut};

mod util;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

extern "C" fn write_cb(_: *mut c_void, message: *const c_char) {
    print!("{}", String::from_utf8_lossy(unsafe {
        std::ffi::CStr::from_ptr(message as *const i8).to_bytes()
    }));
}

fn main() -> Result<(), Error> {
    unsafe { jemalloc_sys::malloc_stats_print(Some(write_cb), null_mut(), null()) };

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
