extern crate curl;
fn main(){
    use std::io::{stdout, Write};

    use curl::easy::Easy;

// Write the contents of rust-lang.org to stdout
    let mut easy = Easy::new();
    easy.url("https://www.rust-lang.org/").unwrap();
    easy.write_function(|data| {
        Ok(stdout().write(data).unwrap())
    }).unwrap();
    easy.perform().unwrap();
}