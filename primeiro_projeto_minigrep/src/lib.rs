use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {

    //método/construtor recebe um slice de string (uma lista de referencias de string)
    // e retorna um Result<Config, String Literal>
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        /* 👆👆 é necessário fazer o clone porque se só
         atribuirmos o valor de args[1], o args[1]
         deixaria de existir, pois estariamos
         movendo nao só o valor, mas tambem a
         referencia, conforme visto em:
         ./ownership/ownership.rs linha 47
         além disso, args é um slice, e o rust
         slices não podem ser movidos */

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), io::Error> {
    let mut f = File::open(config.filename)?; //'?' significa se is_err = true, lança panic, se não segue

    let mut contents = String::new();
    f.read_to_string(&mut contents)?; //'?' significa se is_err = true, lança panic, se não segue

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

/* como a função recebe 2 parametros de referencia de
string e retorna 1 vetor de referencia de string
o rust nao sabe qual tempo de vida do retorno
se é o mesmo do parametro 1 ou 2, se só tivesse 1
parametro nao haveria problema, com 'a estou avisando
o rust que o tempo de vida no retorno é o mesmo de contents

Porque contents é o argumento que contém tod0 o nosso texto
e nós queremos retornar as partes desse texto que combinam,
sabemos que o contents é o argumento que deve ser conectado
ao valor de retorno usando a sintaxe de lifetime*/
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results

    /* código equivalente usando iterator adaptor:
    pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        contents
            .lines()
            .filter(|line| line.contains(query))
            .collect()
    } */
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
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
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\n
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn three_results() {
        let query = "inha";
        let contents = "\
godzilla vs kong
caju e castanhazinha
bruxa do 71
casa magrelinha
brasil
costelinha";

        let expected = vec!["caju e castanhazinha",
                                        "casa magrelinha", "costelinha"];
        let result = search(query, contents);

        assert_eq!(expected, result);
    }

    #[test]
    fn config_new_should_return_config() {
        let args: Vec<String> = vec![String::from("local"),
                                     String::from("query"),
                                     String::from("filename")];

        let result = Config::new(&args);
        assert!(!result.is_err());

        let config = result.unwrap();
        assert_eq!("query", config.query);
        assert_eq!("filename", config.filename);
    }

    #[test]
    fn config_new_should_return_error() {
        let args: Vec<String> = vec![String::from("local")];

        let result = Config::new(&args);

        assert!(result.is_err());
        assert_eq!("not enough arguments", result.err().unwrap());
    }

    #[test]
    fn run_should_return_ok() {
        let args: Vec<String> = vec![String::from("local"),
                                     String::from("frog"),
                                     String::from("poema.txt")];

        let result = Config::new(&args);
        let config = result.unwrap();

        let result = run(config);

        assert!(result.is_ok());
        assert!(!result.is_err());
    }

    #[test]
    fn run_should_return_panic_when_try_open_file() {
        let args: Vec<String> = vec![String::from("local"),
                                     String::from("frog"),
                                     String::from("file.txt")];

        let result = Config::new(&args);
        let config = result.unwrap();

        let result = run(config);

        assert!(result.is_err());
        assert!(!result.is_ok());
        assert_eq!("No such file or directory (os error 2)", result.err().unwrap().to_string());
    }
}