use std::path::{Path, PathBuf};

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

    #[clap(short = 'd', long = "depth")]
    depth: Option<usize>,

    #[clap(long = "with-node-modules")]
    with_node_modules: bool,
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
    let config = RenderConfig {
        depth: args.depth,
        with_node_modules: args.with_node_modules,
    };
    handle_path(path, &resolver, &mut graph, &config);

    println!("{}", graph.to_dot());
}

fn handle_path(path: &Path, resolver: &Resolver, graph: &mut Graph, config: &RenderConfig) {
    if path.is_file() {
        let imports = match get_imports(path, &resolver) {
            Ok(imports) => imports,
            Err(e) => {
                eprintln!("{}", e);
                return;
            }
        }
        .iter()
        .filter_map(|path| render(path, resolver, config))
        .collect();
        render(path, resolver, config).map(|path| graph.add_file(&path, imports));
    } else if path.is_dir() {
        for file in path.read_dir().expect("Could not read directory") {
            match file {
                Ok(file) => handle_path(&file.path(), resolver, graph, config),
                Err(e) => eprintln!("{:?}", e),
            }
        }
    } else {
        panic!("Unexpected argument");
    }
}

#[derive(Debug)]
struct RenderConfig {
    depth: Option<usize>,
    with_node_modules: bool,
}

fn render<'a>(path: &'a Path, resolver: &Resolver, config: &RenderConfig) -> Option<PathBuf> {
    let result = resolver.relative_to_project_root(path);

    if !config.with_node_modules && result.starts_with("node_modules") {
        return None;
    }

    let mut result = result.display().to_string();

    if let Some(depth) = config.depth {
        let cut: Vec<_> = result.split("/").take(depth).collect();
        result = cut.join("/");
    }

    Some(PathBuf::from(result))
}
