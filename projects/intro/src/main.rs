// mod number_guessing_game;
// use number_guessing_game::guessing_game;

mod greatest_common_divisor;
use greatest_common_divisor::gcd;

// fn main() {
//     // guessing_game()
//     let x = gcd(500250, 1000225);
//     println!("{}", x)
// }

use std::io::Write;
use std::str::FromStr;

fn main() {
    let mut numbers = Vec::new();
    for arg in std::env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"))
    }

    if numbers.len() == 0 {
        writeln!(std::io::stderr(), "Usage: gcd Number ...").unwrap();
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }
    println!("The greatest common divisor of {:?} is {}", numbers, d);
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1)
}
