struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    println!("Hello world!");
    let pattern = std::env::args().nth(1).expect("No pattern given!! ");
    let path = std::env::args().nth(2).expect("No path given!! ");
    // println!("{}",pattern);
    // println!("{}",path);
    let _args = Cli {
        pattern: pattern,
        path: std::path::PathBuf::from(path),
    };
}
