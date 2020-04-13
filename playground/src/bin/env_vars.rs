fn main() {
    let version = env!("CARGO_PKG_VERSION_PATCH")
        .parse::<u8>()
        .expect("parse success");
    println!("version = {}", version);
    let dash_pre = {
        let pre = env!("CARGO_PKG_VERSION_PRE");
        if pre == "" {
            pre.to_string()
        } else {
            "-".to_string() + pre
        }
    };
    println!("version = {}", dash_pre);
    let v = Box::new(5);
}
