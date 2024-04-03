use configurations;
fn main() {
    let (conf, opt) = configurations::parse();
    println!("conf: {:?}", conf);
    println!("opt: {:?}", opt);
}
