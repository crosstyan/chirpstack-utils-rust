use clap::Parser;

// two slash is the normal comment
// three slash is thee doc comment

/// Simple program to greet a person
// https://doc.rust-lang.org/rust-by-example/trait/derive.html
// https://doc.rust-lang.org/rust-by-example/attribute.html
#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// Name of the person to greet
    // the short (-n) and long (-name) flags are auto generated
    #[clap(short, long)]
    name: String,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}