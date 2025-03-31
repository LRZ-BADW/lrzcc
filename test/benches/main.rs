#[macro_use]
extern crate bencher;

use std::{env, str::FromStr};

use bencher::Bencher;
use lrzcc::{Api, Token};

fn bench_hello_user(b: &mut Bencher) {
    let token =
        Token::from_str(env::var("OS_TOKEN").unwrap().as_str()).unwrap();
    let api =
        Api::new("http://localhost:8000/api".to_string(), token, None, None)
            .unwrap();

    b.iter(|| {
        api.hello.user().unwrap();
    });
}

benchmark_group!(benches, bench_hello_user);
benchmark_main!(benches);
