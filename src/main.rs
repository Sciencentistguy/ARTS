use std::fs;
use std::path::Path;

use structopt::StructOpt;

fn main() {
    let args = Opt::from_args();
    println!("{:?}", args);

    let tasks = parse_input(args.input, args.implicit_deadlines);
    println!("{:?}", tasks);

    l_and_l_utilisation(&tasks);
}

fn l_and_l_utilisation(tasks: &[Task]) -> bool {
    let n = tasks.len();
    let limit = n as f64 * ((2 as f64).powf(1.0 / n as f64) - 1.0);
    let sum: f64 = tasks.iter().map(|x| x.C as f64 / x.T as f64).sum();
    let schedulable = sum <= limit;
    println!("U: {}; Limit: {}", sum, limit);
    println!(
        "These tasks {} schedulable according to the L&L Utilisation-based test.",
        if schedulable { "are" } else { "may not be" }
    );
    schedulable
}

fn parse_input<P: AsRef<Path>>(path: P, implicit_deadlines: bool) -> Vec<Task> {
    let input = fs::read_to_string(path).expect("Could not open file");
    println!("{}", input);
    input
        .lines()
        .skip(1)
        .map(|line| {
            println!("{}", line);
            // a well-formed input is in order: name, T, D, C, P (D is ignored in implicit_deadlines mode)
            let mut iter = line.split(',');
            let name = iter.next().expect("Ill-formed input").trim().into();
            let t = iter.next().expect("Ill-formed input").trim().parse().expect("Ill-formed input");
            let d = if implicit_deadlines {
                iter.next();
                t
            } else {
                iter.next().expect("Ill-formed input").trim().parse().expect("Ill-formed input")
            };
            let c = iter.next().expect("Ill-formed input").trim().parse().expect("Ill-formed input");
            let p = iter.next().expect("Ill-formed input").trim().parse().expect("Ill-formed input");

            Task {
                name,
                T: t,
                D: d,
                C: c,
                P: p,
                R: 0,
            }
        })
        .collect()
}

#[derive(Debug)]
#[allow(non_snake_case)]
struct Task {
    name: String,
    T: u64,
    D: u64,
    C: u64,
    P: u64,
    R: u64,
}

#[derive(StructOpt, Debug)]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
#[structopt(name = "arts", about = "A program to do some of the calculations from ARTS")]
pub struct Opt {
    /// The csv file containg the tasks
    #[structopt()]
    input: String,

    /// Whether the deadline should be assumed to be equal to T and ignored
    #[structopt(short, long)]
    implicit_deadlines: bool,
}
