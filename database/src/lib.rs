use std::collections::HashMap;

#[derive(Default)]
pub struct Database {
    urls: HashMap<String, String>,
}

impl Database {
    pub fn new() -> Database {
        let urls = HashMap::from([
            (String::from("/"), String::from("html/hello.html")),
            (String::from("/page1"), String::from("html/page1.html")),
            (
                String::from("/page1/page2"),
                String::from("html/page2.html"),
            ),
            (String::from("/page3"), String::from("html/page3.html")),
            (String::from(""), String::from("html/404.html")),
            (String::from("/favicon.ico"), String::from("")),
        ]);
        Database { urls }
    }
    pub fn get(&self, key: &str) -> &str {
        self.urls.get(key).map(String::as_str).unwrap()
    }
}
