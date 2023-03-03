mod args;
mod helper;
mod report;
mod hands;

use clap::Parser;
use args::*;
use report::*;
fn main() {
    let args=DsolverArgs::parse();
    let mut report=Report::new();
    report.print_report(args);
}
