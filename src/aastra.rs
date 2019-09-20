use clap::ArgMatches;
use reqwest;
use std::str::FromStr;
use std::time::Duration;
use std::time::Instant;
use url::Url;

#[derive(Debug, Eq, PartialEq)]
struct RequestStats {
    time_taken: Duration,
    response_code: u16,
}
#[derive(Debug, Eq, PartialEq)]
pub struct Aastra {
    url: Url,
    num_req: u16,
    req_stats: Vec<RequestStats>,
}

impl Aastra {
    pub fn new(matches: ArgMatches) -> Aastra {
        Aastra {
            url: Url::parse(matches.value_of("url").unwrap()).unwrap(),
            num_req: u16::from_str(matches.value_of("number").unwrap_or("10")).unwrap(),
            req_stats: Vec::new(),
        }
    }

    pub fn run_sync_serially(&mut self) {
        for i in 0..self.num_req {
            let t = Instant::now();
            let res = reqwest::blocking::get(self.url.as_str()).unwrap();
            let stats = RequestStats {
                time_taken: t.elapsed(),
                response_code: res.status().as_u16(),
            };
            println!("Call no: {}", i + 1);
            println!("Status: {}", &stats.response_code);
            println!("TimeTaken : {:#?}", &stats.time_taken);
            self.req_stats.push(stats);
        }
        dbg!(&self.req_stats);
    }
}
