#[allow(unused_imports)]
use log::{info, error, debug, warn, trace};

use structopt::StructOpt;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::error::Error;
use std::fmt;
use dotenv::{dotenv, from_filename};

pub fn parse()-> (Config, Opt){
    dotenv().ok();
    let opt = Opt::from_args();
    let conf = Config::load_yaml_with_opt_override(&opt).unwrap();
    (conf, opt)
}

pub fn parse_with_env(env_file_name: &str)-> (Config, Opt){
    from_filename(env_file_name).ok();
    let opt = Opt::from_args();
    let conf = Config::load_yaml_with_opt_override(&opt).unwrap();
    (conf, opt)
}

#[derive(Debug)]
pub struct ConfigErr{
    err: String,
}
impl ConfigErr{
    pub fn new(err: &str) -> Box<Self> {
        Box::new(ConfigErr{err: err.to_string()})
    }
}
impl fmt::Display for ConfigErr{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(write!(f, "{}", self.err)?)
    }
}
impl Error for ConfigErr{}


#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Config{
    version: usize,

    /// Whether to run this process in the background.
    pub daemon: bool,

    pub nonoverride: i8,
    // override is a reserved keyword in Rust, so add a prefix
    pub r#override: i8,
}

impl Default for Config{
    fn default() -> Self {
        Config{
            version: 0,
            daemon: false,
            nonoverride: 0,
            r#override: 0,
        }
    }
}


/// Call `Opt::from_args()` to build this object from the process's command line arguments.
#[derive(StructOpt, Debug)]
#[structopt(
    name = "my-program-name",
    version = "0.1.4",
    about = "description for your command"
)]
pub struct Opt{
    
    /// `-d` or `--daemon` can be used
    #[structopt(short, long)]
    pub daemon: bool,

    #[structopt(short, long, env="LOG_LEVEL", default_value="1")]
    pub log_level: i8,

    /// `-t` or `--test` can be used
    #[structopt(
        short = "t",
        long = "test",
        env = "TESTMODE",
        takes_value=false,
        help = "toggle test mode",
    )]
    pub test: bool,

    #[structopt(
        short = "T",
        long = "no-test",
        conflicts_with="test",
        name="notest",
    )]
    pub notest: bool,

    /// `-c` or `--conf` can be used multiple times
    #[structopt(
        short, 
        long,
        value_name = "YMLFILE", // https://docs.rs/clap/2.34.0/clap/struct.Arg.html#method.value_name
        parse(from_os_str),
    )]
    pub conf: Vec<PathBuf>,

}

impl Default for Opt {
    fn default() -> Self {
        Opt::from_args()
    }
}

fn merge_yaml(a: &mut serde_yaml::Value, b: serde_yaml::Value) {
    match (a, b) {
        (a @ &mut serde_yaml::Value::Mapping(_), serde_yaml::Value::Mapping(b)) => {
            let a = a.as_mapping_mut().unwrap();
            for (k, v) in b {
                if v.is_sequence() && a.contains_key(&k) && a[&k].is_sequence() { 
                    let mut _b = a.get(&k).unwrap().as_sequence().unwrap().to_owned();
                    _b.append(&mut v.as_sequence().unwrap().to_owned());
                    a[&k] = serde_yaml::Value::from(_b);
                    continue;
                }
                if !a.contains_key(&k) {a.insert(k.to_owned(), v.to_owned());}
                else { merge_yaml(&mut a[&k], v); }

            }
            
        }
        (a, b) => *a = b,
    }
}


impl Config{
    // Does not has to be async until we want runtime reload
    pub fn load_from_yaml<P>(path: P) -> Result<Self, Box<dyn Error>>
    where
        P: AsRef<std::path::Path> + std::fmt::Display,
    {
        let conf_str = fs::read_to_string(&path)?;
        debug!("Conf file read from {path}");
        Self::from_yaml(&conf_str)
    }

    pub fn load_yaml_with_opt_override(opt: &Opt) -> Result<Self, Box<dyn Error>> {
        if opt.conf.len() == 0 {
            return Err(ConfigErr::new("No path specified"));
        }
        let mut target_yml: serde_yaml::Value = serde_yaml::from_str("---\nversion: 1")?;
        // 实现多个 yaml 文件合并的效果
        for ymlpath in &opt.conf {
            let conf_str = fs::read_to_string(&ymlpath)?;
            let val : serde_yaml::Value = serde_yaml::from_str(&conf_str)?;
            merge_yaml(&mut target_yml, val);
        }
        let mut conf: Self = serde_yaml::from_value(target_yml)?;

        if opt.daemon {
            conf.daemon = true;
        }
        Ok(conf)
    }

    pub fn new() -> Option<Self> {
        Self::from_yaml("---\nversion: 1").ok()
    }

    pub fn new_with_opt_override(opt: &Opt) -> Option<Self> {
        let conf = Self::new();
        match conf {
            Some(mut c) => {
                if opt.daemon {
                    c.daemon = true;
                }
                Some(c)
            }
            None => None,
        }
    }

    pub fn from_yaml(conf_str: &str) -> Result<Self, Box<dyn Error>> {
        trace!("Read conf file: {conf_str}");
        let conf: Self = serde_yaml::from_str(conf_str)?;
        trace!("Loaded conf: {conf:?}");
        conf.validate()
    }

    pub fn to_yaml(&self) -> String {
        serde_yaml::to_string(self).unwrap()
    }

    pub fn validate(self) -> Result<Self, Box<dyn Error>> {
        // TODO: do the validation
        Ok(self)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn init_log() {
        let _ = env_logger::builder().is_test(true).try_init();
    }


    #[test]
    fn test_load_file() {
        init_log();
        let conf_str = r#"
---
version: 1
daemon: true
        "#
        .to_string();
        let conf = Config::from_yaml(&conf_str).unwrap();
        assert_eq!(true, conf.daemon);
        assert_eq!(1, conf.version);
    }
}
