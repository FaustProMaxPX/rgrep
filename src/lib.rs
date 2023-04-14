use std::{error::Error, fs, env};

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    
    // fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        
    // }

    // 这里可以直接将一个迭代器传进来，因为是用于初始化的参数，所以所有权移交进来也没什么关系
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Can not get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Can not get a file path"),
        };

        // 读取环境变量，检测是否要忽略大小写
        
        let ignore_var = env::var("IGNORE_CASE").expect("读取环境变量失败");
        let ignore_case = ignore_var.parse().expect("无法将入参转为布尔型");
        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn exec(config: Config) -> Result<(), Box<dyn Error>> {
    // ?的作用：如果处理成功会将结果赋给contents，否则会构造一个错误返回
    let contents = fs::read_to_string(&config.file_path)?;
    for ele in search(&config.query, &contents) {
        println!("{}", ele);
    }
    Ok(())
}

// 这里需要标注生命周期，因为返回值包含了引用，并且编译器无法通过生命周期消除推断出生命周期是否符合要求
pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.trim().contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut ret = Vec::new();
    // 用变量遮蔽隐藏传入的query
    let query = query.to_lowercase();
    for ele in contents.lines() {
        if ele.trim().contains(&query) {
            ret.push(ele.trim());
        }
    }
    return ret;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "safe";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn insensitive_result() {
        let query = "SAfe";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three";
        assert_eq!(vec!["safe, fast, productive."], search_case_insensitive(query, contents));
    }
}