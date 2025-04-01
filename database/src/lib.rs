use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::{self, BufRead, BufReader, Write},
    path::PathBuf,
};

const DATAFILE: &str = "database/urls.txt";

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
        self.urls.get(url).unwrap()
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
        eprintln!("urls.txt could not be found");
        Ok(())
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
        self.save()
    }
    pub fn remove(&mut self, path: &str) -> io::Result<()> {
        let target = "urls.txt";
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.file_name().map(|name| name == target).unwrap_or(false) {
                return self.remove_from_urlsfile(path);
            }
        }
        eprintln!("could not find urls.txt file");
        Ok(())
    }
    fn remove_from_urlsfile(&mut self, file: PathBuf) -> io::Result<()> {
        let urlsfile = File::open(file)?;

        for line in BufReader::new(urlsfile).lines() {
            let line = line?;
            let mut kv = line.split_whitespace();

            if let Some(k) = kv.next() {
                self.urls.remove(k);
            }
        }
        self.save()
    }
    pub fn contains(&self, url: &str) -> bool {
        self.urls.contains_key(url)
    }
    ///clears the data base
    pub fn clear(&mut self) -> io::Result<()> {
        self.urls.clear();
        self.urls.insert(String::from(""), String::from("404.html"));
        self.save()
    }
    pub fn save(&self) -> io::Result<()> {
        let mut file_map = HashMap::new();
        if let Ok(file) = File::open(DATAFILE) {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                let line = line?;
                if let Some((key, value)) = line.split_once('=') {
                    file_map.insert(key.to_string(), value.to_string());
                }
            }
        }

        // Update the file_map with only new or changed entries from the input hashmap
        for (key, value) in &self.urls {
            if file_map.get(key) != Some(value) {
                // Only update if the value is different or does not exist
                file_map.insert(key.clone(), value.clone());
            }
        }

        // Remove entries from file_map that are not in the new hashmap
        file_map.retain(|key, _| self.contains(key));

        // Write updated data back to the file
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(DATAFILE)?;

        for (key, value) in &file_map {
            writeln!(file, "{}={}", key, value)?;
        }

        Ok(())
    }
    pub fn load() -> Database {
        let mut urls = HashMap::new();
        if let Ok(file) = File::open(DATAFILE) {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                let line = line.unwrap();
                if let Some((key, value)) = line.split_once('=') {
                    urls.insert(key.to_string(), value.to_string());
                }
            }
        }
        Database { urls }
    }
    pub fn refresh(&mut self) {
        *self = Database::load();
    }

    pub fn help() {
        println!("valid arguments");
        println!(
            "--add paths... - adds urls from the paths given. paths must contain a url.txt file"
        );
        println!(
            "remove paths... - removes urls from the [paths] given, paths must contain a url.txt file"
        );
        println!("clear - removes all urls from the database");
        println!("help - shows this message");
    }
}
