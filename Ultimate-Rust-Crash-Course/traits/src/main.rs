trait Noisy {
    fn get_noise(&self) -> &str;
}

fn print_noise<T: Noisy>(item: T) {
    println!("{}", item.get_noise());
}

impl Noisy for u8 {
    fn get_noise(&self) -> &str {
        "BYTE!"
    }
}

fn main() {
    print_noise(5_u8);
}