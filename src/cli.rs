// mod lexer;
// mod interpreter;

use std::fs::File;
use std::io::prelude::*;
use std::string::String;
use std::error::Error;
// use std::path::PathBuf


pub fn start_nano()->Result<String,Box<dyn Error>>{
    /*Starts nano */
    let mut banner_string = String::new();
    let mut banner_file = File::open("src/banner.txt")?;    
    banner_file.read_to_string(&mut banner_string)?;
    Ok(banner_string)    
}
pub fn loopss(){
    
}
