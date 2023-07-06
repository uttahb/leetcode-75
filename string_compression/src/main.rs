use std::num;

fn main() {
    println!("Hello, world!");
    let mut chars = vec!['a','a','b','b','c','c','c'];
    println!("{}", compress(&mut chars));
}
fn compress(chars: &mut Vec<char>) -> i32 {
    let new_chars = chars.clone();
    chars.clear();
    let mut last_char = new_chars[0];
    let mut last_count = 0;
    let mut index = 0;
    let max_index = new_chars.len() - 1;
    fn count_to_chars(last_count: i32, characters: &mut Vec<char>) {
        if last_count != 1 {
            let num_string = last_count.to_string();
            let num_chars = num_string.chars();
            for j in num_chars {
                characters.push(j);
            }
        }
    }
    for i in new_chars {
        println!("last_count is {} for char {}",last_count,i);
        if i != last_char {
            chars.push(last_char);
            count_to_chars(last_count, chars);
            if index == max_index {
                chars.push(i);
            }
            last_char = i;
            last_count = 1;

        }else {
            last_count += 1;
            if index == max_index {
                chars.push(last_char);
                count_to_chars(last_count, chars);
            }
        }
        index += 1;
    }
    println!("{:?}",chars);
    return chars.len() as i32;
}
