mod day1 {
    mod lib;

    pub use lib::Day1;
}

use day1::Day1;

fn main() {
    Day1::run();
}