use std::fs;
use std::env;
use std::error::Error;


pub struct Config{
    query: String,
    filename: String,
    case_sensitive: bool
}
impl Config{
    pub fn new(args: &[String]) ->Result<Config, &'static str>{
        if args.len() < 3{
            return Err("not enough arguments")
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_SENSITIVE").is_err();
        Ok(Config{
            query,
            filename,
            case_sensitive,
        })
    }
}


pub fn run(config: Config)->Result<(), Box<dyn Error>>{
    let content = fs::read_to_string(config.filename)?;
    if config.case_sensitive == true{
        for line in search(&config.query, &content){
            println!("{}", line);
        }
    }else{
        for line in search_case_insensitive(&config.query, &content){
            println!("{}", line);
        }
    }

    Ok(())
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str)->Vec<&'a str>{
    let mut result = vec![];
    let query = query.to_lowercase();
    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            result.push(line);
        }
    }
    return result;
}

pub fn search<'a>(query: &str, contents: &'a str)->Vec<&'a str>{
    let mut result = vec![];
    for line in contents.lines(){
        if line.contains(query){
            result.push(line);
        }
    }
    return result;
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn one_result(){
        let query = "pick";
        let contents = "rust safe, fast, productive \n\
        pick three\
        ";
        assert_eq!(search(query, contents), vec!["pick three"]);
    }
}

