#[macro_use]
extern crate clap;

fn main() {
let matches = clap_app!(Indraaastra => 
    (@arg number: -n  +takes_value "number of requests")
    (@arg url: +required  "http[s]://]hostname[:port]/path")).get_matches();
    if let Some(n) = matches.value_of("number"){
    println!("{}",n);
    }
    println!("{}",matches.value_of("url").unwrap());
}
