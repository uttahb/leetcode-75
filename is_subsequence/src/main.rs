fn main() {
    let a = String::from("abc");
    let b = String::from("asdfsdfvcb");
    println!("{}", is_subsequence(a, b));
}
fn is_subsequence(s: String, t: String) -> bool {
    let s_len = s.len();
    if s_len == 0 {
        return false;
    }
    let mut s_index = 0;
    let s_vec = s.chars().collect::<Vec<char>>();
    for i in t.chars() {
        if i == s_vec[s_index] {
            s_index += 1;
            if s_index == s_len {
                return true;
            }
        }
    }
    return false;
}
