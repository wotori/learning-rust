// mod number_guessing_game;
// use number_guessing_game::guessing_game;

mod greatest_common_divisor;
use greatest_common_divisor::gcd;

fn main() {
    // guessing_game()
    let x = gcd(500250, 1000225);
    println!("{}", x)
}
