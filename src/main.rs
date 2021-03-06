use structopt::StructOpt;
#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path).expect("File not found..");
    println!("{}", content);
    println!("`");
    println!("`");
    println!("`");
    println!("`");
    println!("`");
    println!("`");
    // for line in content.lines() {
    //     if line.contains(&args.pattern) {
    //         println!("{}", line);
    //         println!("............");
    //     }
    // }
    println!("`");
    println!("`");
    let result = std::fs::read_to_string(&args.path);
    let content = match result {
        Ok(content) => content,
        Err(error) => {
            panic!("Cant deal with {} ,exiting here ", error);
        }
    };
    println!("{}", content);
}
