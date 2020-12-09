use std::env;

fn main() {
    let args:Vec<String> = env::args().skip(1).collect();
    if args.len() != 1 {
        println!("usage: day_04 input_file");
    } else {
        run(&args[0]);
    }
}

fn run(input_fn: &String) {
    println!("reading {}", input_fn);
}
