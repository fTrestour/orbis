use std::path::Path;

use clap::Parser;
use orbis::get_imports;

#[derive(Parser, Debug)]
struct Args {
    #[arg()]
    file: String,
}

fn main() {
    let args = Args::parse();

    let path = Path::new(&args.file);
    let imports = get_imports(path);
    for import in imports {
        println!("{}", import);
    }
}
