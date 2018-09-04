/*
[dependencies]
curl = "0.4.14"
*/
extern crate curl;
use std::io::{stdout,Write};
use curl::easy::Easy;

fn main() {
    let mut handle = Easy::new();
    handle.url("https://ctftime.org/event/list/upcoming?year=2018&online=1&format=1&restrictions=0&upcoming=true").unwrap();
    handle.write_function(|data| {

        stdout().write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();
    handle.perform().unwrap();
}
