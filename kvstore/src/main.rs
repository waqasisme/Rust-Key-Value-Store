use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().expect("Key was not there");
    let value = arguments.next().expect("value missing");
    println!("The key is '{}', the value is '{}'", key, value);

    let contents = format!("{}\t{}\n", key, value);
    // empty tuple = unit (similar to void)

    let database = Database::new().expect("Database::new() crashed");
    
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        // read kv.db, parse and populate map
        // let contents = match std::fs::read_to_string("kv.db") {
        //     Ok(c) => c>,
        //     Err(e) => {
        //         return Err(e);
        //     }
        // };
        let mut map = HashMap::new();
        let contents = std::fs::read_to_string("kv.db")?;
        for line in contents.lines() {
            let (key, value) = line.split_once('\t').expect("Corrupt Database!");
            map.insert(key.to_owned(), value.to_owned());
        }

        Ok(Database { map })
    }
}
