use clap::ArgMatches;
use reqwest;
use std::str::FromStr;
use std::time::Instant;
use url::Url;
#[derive(Debug, Eq, PartialEq)]
pub struct Aastra {
    url: Url,
    num_req: u16,
}

impl Aastra {
    pub fn new(matches: ArgMatches) -> Aastra {
        Aastra {
            url: Url::parse(matches.value_of("url").unwrap()).unwrap(),
            num_req: u16::from_str(matches.value_of("number").unwrap_or("10")).unwrap(),
        }
    }

    pub fn run(&self) {
        for i in 0..self.num_req {
            let t = Instant::now();
            let res = reqwest::get(self.url.as_str()).unwrap();
            println!("Call no: {}", i);
            println!("Status: {}", res.status());
            println!("TimeTaken : {:#?}", t.elapsed());
        }
    }
}
