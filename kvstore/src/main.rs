use std::collections::HashMap;

fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().expect("Key was not there");
    let value = arguments.next().expect("value missing");
    println!("The key is '{}', the value is '{}'", key, value);
    // empty tuple = unit (similar to void)

    let mut database = Database::new().expect("Database::new() crashed");
    database.insert(key.to_uppercase(), value.clone());
    database.insert(key, value);
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        // read kv.db, parse and populate map
        let mut map = HashMap::new();
        if !std::path::Path::new("kvstore.db").exists() {
            std::fs::write("kvstore.db", "")?;
        }
        let contents = std::fs::read_to_string("kvstore.db")?;
        for line in contents.lines() {
            let (key, value) = line.split_once('\t').expect("Corrupt Database!");
            map.insert(key.to_owned(), value.to_owned());
        }

        Ok(Database { map })
    }

    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    fn flush(&self) -> std::io::Result<()> {
        // not taking self by reference so that flush is the last thing called
        let mut contents = String::new();
        for (key, value) in &self.map {
            contents.push_str(&format!("{}\t{}\n", key, value));
        }
        std::fs::write("kvstore.db", contents)
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        let _ = self.flush();
    }
}

