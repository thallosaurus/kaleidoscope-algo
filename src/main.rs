use clap::Parser;

use crate::cli::CliArgs;

mod cli;

fn main() {
    let args = CliArgs::parse();
    println!("{:?}", args);

    // blender -b -Y -P loader.py $PWD/kaleido.blend -f 0 -- 'aaa aaaaaaaa'
}
