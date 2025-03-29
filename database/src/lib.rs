use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

const DATAFILE: &str = "database/datafile.txt";

#[derive(Default)]
pub struct Database {
    urls: HashMap<String, String>,
}

impl Database {
    pub fn new() -> Self {
        let urls = HashMap::from([(String::from(""), String::from("404.html"))]);
        Self { urls }
    }
    pub fn get(&self, url: &str) -> &str {
        self.urls
            .get(url)
            .unwrap_or_else(|| self.urls.get("").unwrap())
    }
    ///add a file path to the database file which contains urls for the
    ///websites in that path
    ///todo update to handle paths without url.txt files
    pub fn add(&mut self, path: &str) -> io::Result<()> {
        let target = "urls.txt";
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.file_name().map(|name| name == target).unwrap_or(false) {
                return self.add_from_urlsfile(path);
            }
        }
        self.add_from_path(path)
    }

    fn add_from_urlsfile(&mut self, file: PathBuf) -> io::Result<()> {
        let urlsfile = File::open(file)?;

        for line in BufReader::new(urlsfile).lines() {
            let line = line?;
            let mut kv = line.split_whitespace();

            if let (Some(k), Some(v)) = (kv.next(), kv.next()) {
                if !self.contains(k) {
                    self.urls.insert(k.to_string(), v.to_string());
                }
            } else {
                eprintln!("line can not be turned a url and a path {line}");
            }
        }
        Ok(())
    }
    fn add_from_path(&self, path: &str) -> io::Result<()> {
        Ok(())
    }
    pub fn remove(&self, path: &str) {}
    pub fn contains(&self, url: &str) -> bool {
        self.urls.contains_key(url)
    }
    ///clears the data file
    pub fn clear() {}
    pub fn save(&self) {}
    pub fn load(&self) {}
}
