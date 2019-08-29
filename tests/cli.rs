#[cfg(test)]
mod cli {
    use assert_cmd::prelude::*;
    use std::process::Command;
    const BINARY_NAME: &str = "indraastra";

    #[test]
    fn no_args() {
        Command::cargo_bin(BINARY_NAME).unwrap().assert().failure();
    }

    #[test]
    fn invalid_url_arg() {
        Command::cargo_bin(BINARY_NAME)
            .unwrap()
            .args(&["url"])
            .assert()
            .failure();
    }

    #[test]
    fn empty_url_arg() {
        Command::cargo_bin(BINARY_NAME)
            .unwrap()
            .args(&[""])
            .assert()
            .failure();
    }

    #[test]
    fn valid_url_arg() {
        Command::cargo_bin(BINARY_NAME)
            .unwrap()
            .args(&["https://google.com/"])
            .assert()
            .success();
    }

    #[test]
    fn invalid_num_req() {
        Command::cargo_bin(BINARY_NAME)
            .unwrap()
            .args(&["https://www.google.com", "-n", "a50"])
            .assert()
            .failure();
    }

    #[test]
    fn empty_num_req() {
        Command::cargo_bin(BINARY_NAME)
            .unwrap()
            .args(&["https://www.google.com", "-n", ""])
            .assert()
            .failure();
    }

    #[test]
    fn valid_num_req() {
        Command::cargo_bin(BINARY_NAME)
            .unwrap()
            .args(&["https://www.google.com", "-n", "50"])
            .assert()
            .success();
    }
}
