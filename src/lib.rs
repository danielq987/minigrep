use std::{env, error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents, config.is_case_sensitive) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str, case_sensitive: bool) -> Vec<&'a str> {
    let mut matched_strings = Vec::new();

    let mut _query = &query;
    let mut _contents = &contents;

    let q: &str = &query.to_lowercase();
    let c: &str = &contents.to_lowercase();

    if !case_sensitive {
        _query = &q;
        _contents = &c;
    }

    for line in (*_contents).lines() {
        if line.contains(*_query) {
            matched_strings.push(line);
        }
    }
    matched_strings
}

pub struct Config {
    query: String,
    filename: String,
    is_case_sensitive: bool,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = &args[1];
        let filename = &args[2];

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query: query.into(),
            filename: filename.into(),
            is_case_sensitive: case_sensitive,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn new_config_2_strings() {
        let arg0 = "arg0".to_string();
        let arg1 = "arg1".to_string();
        let arg2 = "arg2".to_string();

        let input: Vec<String> = vec![arg0, arg1, arg2];
        let config = Config::new(&input);
        assert!(!config.is_err());
        let config: Config = config.unwrap();
        assert_eq!(config.query, input[1]);
        assert_eq!(config.filename, input[2]);
    }

    // #[test]
    // fn new_config_1_string() -> Result<(), String> {
    //     let input: Vec<String> = vec!["arg1".to_string()];
    //     assert!(Config::new(&input).is_err());
    // }

    // #[test]
    // fn new_config_0_string() -> Result<(), String> {
    //     let input = Vec::new();
    //     assert!(Config::new(&input).is_err());
    // }
    fn get_sample_config() -> Config {
        let sample_args: Vec<String> =
            vec!["zero".to_string(), "one".to_string(), "two".to_string()];
        return Config::new(&sample_args).unwrap();
    }

    #[test]
    fn run_invalid_file() {
        let result = run(get_sample_config());
        assert!(result.is_err());
    }

    #[test]
    fn case_insensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
duct three.
Duct tape.";
        assert_eq!(vec!["duct three.", "Duct tape."], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
