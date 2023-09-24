use std::path::Path;

use clap::Parser;
use orbis::get_imports;

#[derive(Parser, Debug)]
struct Args {
    #[arg()]
    path: String,
}

fn main() {
    let args = Args::parse();

    show_path_imports(Path::new(&args.path))
}

fn show_path_imports(path: &Path) {
    if path.is_file() {
        let imports = get_imports(path);

        show_dependencies(path, &imports);
    } else if path.is_dir() {
        for file in path.read_dir().expect("Failed to read directory") {
            if let Ok(file) = file {
                show_path_imports(&file.path())
            }
        }
    } else {
        panic!("Unexpected file type")
    }
}

fn show_dependencies(path: &Path, imports: &Vec<String>) {
    println!("{}", path.display());
    for import in imports {
        println!("  {}", import);
    }
}
