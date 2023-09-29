use std::path::{Path, PathBuf};

use clap::Parser;
use orbis::{get_imports, resolver::Resolver};

#[derive(Parser, Debug)]
struct Args {
    #[clap()]
    path: String,

    #[clap(short = 'c', long = "config")]
    ts_config: Option<String>,
}

fn main() {
    let args = Args::parse();

    if let Some(ts_config) = args.ts_config {
        // TODO: Handle no ts
        let ts_config = Path::new(&ts_config);
        let resolver = Resolver::try_from(ts_config).expect("plop");

        let path = Path::new(&args.path);
        if path.is_file() {
            let imports = get_imports(path, &resolver).expect("yolo");
            show_dependencies(path, imports);
        } else {
            panic!("Unexpected file type")
        }
    }
}

fn show_dependencies(path: &Path, imports: Vec<PathBuf>) {
    println!("{}", path.display());
    for import in imports {
        println!("  {}", import.display());
    }
}
