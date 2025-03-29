use std::{
    collections::{HashMap, HashSet},
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, Write},
};

const DATAFILE: &str = "database/datafile.txt";

#[derive(Default)]
pub struct Database {
    urls: HashMap<String, String>,
}

impl Database {
    pub fn new() -> Database {
        let urls = HashMap::from([(String::from(""), String::from("404.html"))]);
        Database { urls }
    }
    pub fn get(&self, key: &str) -> &str {
        self.urls.get(key).map(String::as_str).unwrap()
    }
    ///update the url hashmap to include any new paths in the data base
    pub fn update_urls(&mut self) -> io::Result<()> {
        let datafile = File::open(DATAFILE)?;

        let reader = BufReader::new(datafile);
        for line in reader.lines() {
            let line = line?;
            let mut kv = line.split_whitespace();

            // Extract the key and value from the iterator
            if let (Some(key), Some(value)) = (kv.next(), kv.next()) {
                if self.urls.contains_key(key) {
                    continue;
                }
                self.urls.insert(key.to_string(), value.to_string());
            } else {
                // Handle malformed lines (e.g., lines without both key and value)
                eprintln!("Malformed line: {}", line);
            }
        }

        Ok(())
    }
    ///add a file path to the database file which contains urls for the
    ///websites in that path
    pub fn add(path: &str) -> io::Result<()> {
        let urlsfile = File::open(path)?;
        let reader = BufReader::new(urlsfile);

        let datafile = OpenOptions::new()
            .append(true)
            .create(true)
            .open(DATAFILE)?;

        let existing_lines: HashSet<String> = {
            let datafile_reader = BufReader::new(File::open(DATAFILE)?);
            datafile_reader.lines().map_while(Result::ok).collect()
        };
        for line in reader.lines() {
            let line = line?;
            if !existing_lines.contains(&line) {
                writeln!(&datafile, "{}", line)?
            }
        }
        Ok(())
    }
    pub fn remove(&self, path: &str) {}
}
