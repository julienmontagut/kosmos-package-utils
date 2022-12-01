use crate::apk::Package;
use clap::{arg, Command};
use std::fs::read_dir;

mod apk;

fn main() {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .arg(arg!(-d --directory <directory> "The directory to use"))
        .arg(arg!(--labels "Prints the labels"))
        .arg(arg!(--methods "Print the package methods"))
        .get_matches();

    let directory: &String = matches.get_one("directory").expect("directory is required");
    let print_labels: bool = matches.get_flag("labels");
    let print_methods: bool = matches.get_flag("methods");

    println!("Running in directory: {}", directory);

    let entries = read_dir(directory).expect("Failed to read directory");
    for entry in entries {
        let path = match entry {
            Ok(entry) => entry.path(),
            Err(error) => {
                println!("Failed to read entry: {}", error);
                continue;
            }
        };

        if path.is_dir() {
            let apkbuild_path = path.join("APKBUILD");
            if apkbuild_path.is_file() {
                let package = Package::from_file(&apkbuild_path);

                if print_labels || print_methods {
                    println!("  - \x1b[1mDependencies:\x1b[0m");
                }
                for dependency in package.depends() {
                    println!("    - {}", dependency);
                }

                println!("{} v{}", package.name(), package.version());
                if print_methods {
                    for method in package.methods() {
                        println!("    - {}", method);
                    }
                }
            }
        }
    }
}
