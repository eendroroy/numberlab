use colored::Colorize;
use num_bigint::BigUint;
use numseries::series::fibonacci::nth_fibonacci_with_memoizer;
use std::env;

fn header(text: &str) -> String {
    format!("{}", text.truecolor(162, 190, 140).bold())
}

fn command(text: &str) -> String {
    format!("{}", text.truecolor(143, 188, 187).bold())
}

fn value(text: &str) -> String {
    format!("{}", text.truecolor(135, 192, 208))
}

fn description(text: &str) -> String {
    format!("{}", text.truecolor(216, 222, 233))
}

fn help() {
    println!(
        "{} {} {}",
        header("Usage:"),
        command("fibonacci"),
        value("[options]")
    );
    println!();
    println!("{}", header("Options:"));
    println!(
        "  {}                               {}",
        command("-h, --help"),
        description("Print the help menu")
    );
    println!(
        "  {} {}                      {}",
        command("-c, --count"),
        value("<count>"),
        description("Set the number of Fibonacci numbers to print")
    );
    println!(
        "  {}                        {}",
        command("-p, --print-count"),
        description("Print the count of the Fibonacci number")
    );
    std::process::exit(0);
}

fn fibonacci_sequence_printer(n: usize, print_n: bool) {
    let mut memoizer = vec![BigUint::from(0u128), BigUint::from(1u128)];

    for i in 0..n {
        nth_fibonacci_with_memoizer(i, &mut memoizer);
        if print_n {
            println!(
                "{}",
                format!(
                    "{: >width$}: {}",
                    i + 1,
                    memoizer[i].to_string(),
                    width = 10
                )
            )
        } else {
            println!("{:?}", memoizer[i]);
        }
    }
}

pub fn main() {
    let mut args = env::args().skip(1);

    let mut count = 10_usize;
    let mut print_count = false;

    while let Some(arg) = args.next() {
        match &arg[..] {
            "-h" | "--help" => {
                help();
            }
            "-c" | "--count" => {
                if let Some(count_arg) = args.next() {
                    count = count_arg.parse::<usize>().unwrap();
                } else {
                    panic!("No value specified for parameter --count.");
                }
            }
            "-p" | "--print-count" => {
                print_count = true;
            }
            _ => {
                if arg.starts_with('-') {
                    println!("Unknown argument {}", arg);
                } else {
                    println!("Unknown positional argument {}", arg);
                }
            }
        }
    }

    fibonacci_sequence_printer(count, print_count);

    std::process::exit(0);
}
