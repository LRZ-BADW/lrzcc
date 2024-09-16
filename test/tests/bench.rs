#![feature(test)]

extern crate test;

use test::black_box;
use test::Bencher;

use std::env;
use std::str::FromStr;

use lrzcc::{Api, Token};

#[bench]
fn bench_hello_user(b: &mut Bencher) {
    let token =
        Token::from_str(env::var("OS_TOKEN").unwrap().as_str()).unwrap();
    let api =
        Api::new("http://localhost:8000/api".to_string(), token, None, None)
            .unwrap();

    b.iter(|| {
        for _ in 1..100 {
            black_box(api.hello.user().unwrap());
        }
    });
}
