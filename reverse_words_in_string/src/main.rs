fn main() {
    println!("Hello, world!");
    let a = "this is jisynu   sdfs".to_string();
    println!("{:?}", reverse_words(a));
}
fn reverse_words(s: String) -> String {
    let mut a = s.split(" ").collect::<Vec<&str>>();
    a.retain(|x| x != &"");
    a.reverse();
    println!("{:?}", a);
    return a.join(" ");
}
