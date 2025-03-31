use std::env;
fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 2 {
        panic!("too many args. give 1 ip address or nothing for default");
    }

    let mut ip: Option<&str> = None;
    if args.len() != 1 {
        ip = Some(&args[1]);
    }
    server::run(ip);
}
