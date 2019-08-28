use assert_cmd::prelude::*;
use predicates::boolean::PredicateBooleanExt;
use predicates::str::contains;
use std::process::Command;

const BINARY_NAME: &str = "indraastra";

// Indraastra with no args should fail
#[test]
fn aastra_no_args() {
    Command::cargo_bin(BINARY_NAME).unwrap().assert().failure();
}

// Indrastra with url should print the url
#[test]
fn aastra_url_arg() {
    Command::cargo_bin(BINARY_NAME)
        .unwrap()
        .args(&["url"])
        .assert()
        .success()
        .stdout(contains("url"));
}

// Indrastra with url -n 50 should print the url and 50
#[test]
fn aastra_url_arg_and_number() {
    Command::cargo_bin(BINARY_NAME)
        .unwrap()
        .args(&["url", "-n", "50"])
        .assert()
        .success()
        .stdout(contains("url").and(contains("50")));
}
