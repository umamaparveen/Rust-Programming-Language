use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args.next();

        let query = args.next().ok_or("Didn't get a query string")?;

        let filename = args.next().ok_or("Didn't get a file name")?;

        Ok(Config {
            query,
            filename,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if std::env::var("IGNORE_CASE").is_ok() {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();

    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";

        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        let result = search(query, contents);

        assert_eq!(vec!["safe, fast, productive."], result);
    }

    #[test]
    fn multiple_results() {
        let query = "safe";

        let contents = "\
Rust:
safe, fast, productive.
safe and reliable.
Pick three.";

        let result = search(query, contents);

        assert_eq!(
            vec![
                "safe, fast, productive.",
                "safe and reliable."
            ],
            result
        );
    }
}