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

// READING FILES //

// text to list
pub fn readlines_to_vec(filename: impl AsRef<Path>) -> io::Result<Vec<String>>
{
    BufReader::new(File::open(filename)?).lines().collect()
}
