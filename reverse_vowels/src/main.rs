fn main() {
    println!("Hello, world!");
    let s = "hello".to_string();
    println!("{}", (reverse_vowels(s)));
}
fn reverse_vowels(s: String) -> String {
    let vowels = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    let mut vowelse_in_order = s.chars().filter(|x| vowels.contains(x)).rev();

    let mut rev_str = String::new();
    for i in s.chars() {
        if vowels.contains(&i) {
            let char = vowelse_in_order.next();
            rev_str.push(match char {
                Some(v) => v,
                _ => '\0',
            });
        } else {
            rev_str.push(i)
        }
    }
    return rev_str;
}
