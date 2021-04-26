use structopt::StructOpt;
#[derive(StructOpt)]
struct Cli{
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    println!("Hello world!");
    let args = Cli::from_args();
}
