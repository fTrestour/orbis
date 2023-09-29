use std::path::Path;

use clap::Parser;
use orbis::{get_imports, resolver::Resolver};

#[derive(Parser, Debug)]
struct Args {
    #[clap()]
    path: String,

    #[clap(short = 'c', long = "config")]
    ts_config: Option<String>,

    #[clap(short = 'p', long = "project")]
    project_dir: Option<String>,
}

fn main() {
    let args = Args::parse();

    let resolver = if let Some(ts_config) = args.ts_config {
        let ts_config = Path::new(&ts_config);
        Resolver::try_from(ts_config).expect("plop")
    } else if let Some(project_dir) = args.project_dir {
        Resolver::from_project_url(Path::new(&project_dir))
    } else {
        panic!("--project should be specified if --config is not");
    };

    let path = Path::new(&args.path);
    if path.is_file() {
        let imports = get_imports(path, &resolver).unwrap();

        println!("{}", path.display());
        for import in imports {
            println!("  {}", import.display());
        }
    } else {
        panic!("This tool does not support directories");
    }
}
