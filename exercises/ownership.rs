fn takes_ownership(x: String) {
    println!("Goodbye, {}", x);
}

fn main() {
    let s = String::from("Solana");
    takes_ownership(s);
}
