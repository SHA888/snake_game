fn main() {
    let name = "Tim";
    print_message();
    print_name(name);
}

fn print_name(name: &str) {
    println!("{}", name);
}

fn print_message() {
    let say_hi = "Hi there";
    println!("{}", say_hi);
}