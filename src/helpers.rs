/*
    FILE: Helpers.rs
    AUTHOR(s): electra_rta

    DESCRIPTION: Defines useful helper functions
*/
use std::
{
    fs,
    fs::File,
    path::Path,
    env,
    time::Duration,
    collections::HashMap,
    io,
    io::{prelude::*, BufReader, Write},
};

// TYPEOF //
pub fn print_type_of<T>(_: &T) {
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
        if let Some((key, val)) = line.expect("Cannot Read Line").split_once(",")
        {
            res.insert(key.to_string(), val.to_string());
        }
        else
        {
            panic!()
        };
    }
    return Ok(res);
}