#[macro_use]
extern crate clap;
use url::Url;
use std::str::FromStr;
fn main() {
let matches = clap_app!(Indraaastra => 
    (@arg number: -n  +takes_value validator(validate_num) "number of requests")
    (@arg url: +required  validator(validate_url) "http[s]://]hostname[:port]/path")).get_matches();
    if let Some(n) = matches.value_of("number"){
        println!("{}",n);
    }
    println!("{}",matches.value_of("url").unwrap());
}

fn validate_num(num:String)->Result<(),String> {
    match u16::from_str(&num) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string())
    }
}
fn validate_url(url:String)->Result<(),String>{
   match Url::parse(&url) {
    Ok(_) => Ok(()), 
    Err(e) => Err(e.to_string())
   }
}
