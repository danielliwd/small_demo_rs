use configurations;
fn main() {
    //let (conf, opt) = configurations::parse(); // default to `.env` file
    let (conf, opt) = configurations::parse_with_env("env");
    println!("conf: {:?}", conf);
    println!("opt: {:?}", opt);
}
