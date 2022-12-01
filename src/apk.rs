use std::fs::read_to_string;
use std::path::PathBuf;

pub struct Package {
    // attributes: Vec<String>,
    name: String,
    version: String,
    depends: Vec<String>,
    methods: Vec<String>,
}

impl Package {
    pub fn new(raw: String) -> Package {
        let lines = raw
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.to_string());

        let attributes = lines
            .clone()
            .filter(|line| line.contains("="))
            .collect::<Vec<String>>();

        let name = attributes
            .clone()
            .into_iter().find(|line| line.contains("pkgname"))
            .unwrap().split("=").nth(1).unwrap().to_string();

        let version = attributes
            .clone()
            .into_iter().find(|line| line.contains("pkgver"))
            .unwrap().split("=").nth(1).unwrap().to_string();

        let depends = lines
            .clone()
            .filter(|line| {
                line.starts_with("depends=")
                    || line.starts_with("makedepends=")
                    || line.starts_with("checkdepends=")
            })
            .map(|line| {
                line.split("=")
                    .nth(1)
                    .expect("Failed to get value from line")
                    .to_string()
            })
            .map(|line| line.trim_matches('"').to_string())
            .collect::<Vec<String>>();

        let methods = lines
            .clone()
            .into_iter()
            .filter(|line| line.contains("()"))
            .map(|line| {
                line.split("()")
                    .nth(0)
                    .expect("Failed to get method name")
                    .to_string()
            })
            .collect::<Vec<String>>();

        Package {
            // attributes,
            name,
            version,
            depends,
            methods,
        }
    }

    pub fn from_file(file_path: &PathBuf) -> Package {
        let raw = read_to_string(file_path).expect("Something went wrong reading the file");
        Package::new(raw)
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn version(&self) -> &String {
        &self.version
    }

    pub fn depends(&self) -> &Vec<String> {
        &self.depends
    }

    pub fn methods(&self) -> &Vec<String> {
        &self.methods
    }
}
