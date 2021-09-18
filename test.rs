// This is where I test stuff when I'm not sure if it'll work

fn main() {
    let x: String = String::from("asdf");
    for i in (0..10) {
        println!("{}", x.chars().nth(0).unwrap());
    }

    println!("{}", String::from("-1.345").parse::<f32>().unwrap());

    println!("{}", "asdf".starts_with("as") && "asdferson".contains("asdf"));

    let y = vec![1, 2];
    println!("{}", y[0]);
    println!("{}; {}", y.len(), y[1]);

    println!("6/8: {}", 6/8);
    println!("10/8: {}", 10/8);
    println!("45/8: {}", 45/8);

    println!("Location: 49. Output: Sector {} {}", 49/8, 49%8);
}