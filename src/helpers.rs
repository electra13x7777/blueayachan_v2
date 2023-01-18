/*
    FILE: Helpers.rs
    AUTHOR(s): electra_rta

    DESCRIPTION: Defines useful helper functions
*/
use std::
{
    fs::File,
    path::Path,
    collections::HashMap,
    io,
    io::{prelude::*, BufReader}, borrow::Cow,
};

// TYPEOF //
pub fn print_type_of<T>(_: &T)
{
    println!("{}", std::any::type_name::<T>())
}

// READING FILES //

// text to list
pub fn readlines_to_vec(filename: impl AsRef<Path>) -> io::Result<Vec<String>>
{
    BufReader::new(File::open(filename)?).lines().collect()
}

// reads a csv EX.) key,value\n
pub fn readlines_to_map(filename: impl AsRef<Path>) -> io::Result<HashMap<String, String>>
{
    let mut res: HashMap<String, String> = HashMap::new();
    let br = BufReader::new(File::open(filename)?);
    for line in br.lines()
    {
        let line = line?;
        let (key, val) = line.split_once(',').unwrap();
        res.insert(key.to_string(), val.to_string());
    }
    return Ok(res);
}

pub fn to_lowercase_cow(s: &str) -> Cow<'_, str> {
    if s.chars().all(char::is_lowercase) {
        Cow::Borrowed(s)
    } else {
        Cow::Owned(s.to_lowercase())
    }
}
