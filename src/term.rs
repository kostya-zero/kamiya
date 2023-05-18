pub struct Term;
impl Term {
    pub fn success(msg: &str) {
        println!("\x1b[1m\x1b[92m ✔\x1b[0m\x1b[1m {}\x1b[0m", msg);
    }

    pub fn title(msg: &str) {
        println!("\x1b[1m\x1b[92m ☰\x1b[0m\x1b[1m {}\x1b[0m", msg);
    }

    pub fn sub_message(msg: &str) {
        println!("\x1b[1m\x1b[94m >\x1b[0m\x1b[1m {}\x1b[0m", msg);
    }

    pub fn work(msg: &str) {
        println!("\x1b[1m\x1b[96m ⚙\x1b[0m\x1b[1m {}\x1b[0m", msg);
    }

    pub fn fatal(msg: &str) {
        println!("\x1b[1m\x1b[91m ✗\x1b[0m\x1b[1m {}\x1b[0m", msg);
    }
}
