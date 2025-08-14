use std::env;
use video2binframe::run;

fn main() {
    let args: Vec<String> = env::args().collect();

    run(&args[1], &args[2]);
}
