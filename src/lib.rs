use std::{error::Error, fs, env};

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    // 这里传入的是一个字符串数组的引用，通过索引直接访问时取出的是值
    // 通过get取出的是一个&String
    fn new(args: &[String]) -> Config {
        // 这里使用clone显然会带来一定的性能损耗，但因为配置相关的初始化只有一次，因此相关的性能损耗可以忽略
        let query = args[1].clone();
        let file_path = args[2].clone();
        // 读取环境变量，检测是否要忽略大小写
        
        let ignore_var = env::var("IGNORE_CASE").expect("读取环境变量失败");
        let ignore_case = ignore_var.parse().expect("无法将入参转为布尔型");
        Config { query, file_path, ignore_case }
    }

    // 由于new方法按照惯例是必须成功的，因此我们将附带错误提示的方法放到build中
    pub fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        Ok(Self::new(args))
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
    let mut ret = Vec::new();
    for ele in contents.lines() {
        if ele.trim().contains(query) {
            ret.push(ele.trim());
        }
    }
    return ret;
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