use std::path::Path;

use clap::Parser;
use orbis::{get_imports, graph::Graph, resolver::Resolver};

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

    let mut graph = Graph::default();

    let path = Path::new(&args.path);
    handle_path(path, &resolver, &mut graph);

    println!("{}", graph.to_dot());
}

fn handle_path(path: &Path, resolver: &Resolver, graph: &mut Graph) {
    if path.is_file() {
        let imports = match get_imports(path, &resolver) {
            Ok(imports) => imports,
            Err(e) => {
                eprintln!("{}", e);
                return;
            }
        };
        graph.add_file(path, imports);
    } else if path.is_dir() {
        for file in path.read_dir().expect("Could not read directory") {
            match file {
                Ok(file) => handle_path(&file.path(), resolver, graph),
                Err(e) => eprintln!("{:?}", e),
            }
        }
    } else {
        panic!("Unexpected argument");
    }
}
