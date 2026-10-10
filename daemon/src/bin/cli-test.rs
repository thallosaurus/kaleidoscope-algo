use clap_derive::Parser;

/// Test Program - What happens when you start this program with a negative number as argument?
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct CliTestArgs {

    #[arg(short, long, allow_hyphen_values = true)]
    value: f32
}

/// Parses the test value from the command line and prints the arguments.
fn main() {
    let args = <CliTestArgs as clap::Parser>::parse();
    println!("{:?}", args);
}