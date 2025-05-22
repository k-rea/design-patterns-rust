fn main() {
    if let Err(e) = factory::run() {
        println!("{}", e);
        std::process::exit(1)
    }
}
