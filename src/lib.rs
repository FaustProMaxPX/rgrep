use std::{error::Error, fs};

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    // 这里传入的是一个字符串数组的引用，通过索引直接访问时取出的是值
    // 通过get取出的是一个&String
    fn new(args: &[String]) -> Config {
        // 这里使用clone显然会带来一定的性能损耗，但因为配置相关的初始化只有一次，因此相关的性能损耗可以忽略
        let query = args[1].clone();
        let file_path = args[2].clone();
        Config { query, file_path }
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
    println!("{}", contents);
    Ok(())
}