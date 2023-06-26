fn main() {
    let a = gcd_of_strings_alt(
        "TAUXXTAUXXTAUXXTAUXXTAUXX".to_string(),
        "TAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXX".to_string(),
    );
    println!("{}", a);
}

fn gcd_of_strings_alt(str1: String, str2: String) -> String {
    let len_str2 = str2.len();
    let len_str1 = str1.len();
    let mut keyword_length = len_str2;
    let mut str_length = len_str1;
    let mut keyword = String::new();
    let mut str = String::new();

    if len_str1 < len_str2 {
        keyword = str1;
        str = str2;
        keyword_length = len_str1;
        str_length = len_str2;
    } else {
        keyword = str2;
        str = str1;
    }

    for i in (1..keyword_length + 1).rev() {
        if &keyword[0..i] == &str[0..i] && str_length % i == 0 {
            let new_str = &str.replace(&keyword[0..i], "0");
            let new_str_keyword = &keyword.replace(&keyword[0..i], "0");
            // println!("new string is {} for \n{}\n in\n{}",new_str,&keyword[0..i],&str);
            if new_str.parse::<i32>().is_ok() && new_str_keyword.parse::<i32>().is_ok() {
                return keyword[0..i].to_owned();
            }
            println!("{} in {}", &keyword[0..i], &str[0..i]);
        }
    }
    return String::from("");
}
// fn gcd_of_strings(str1: String, str2: String) -> String {
//     fn check_in_str1(str1: &str, str2: String) -> bool {
//         println!("checkstrings are {} in {}", str2, str1);
//         let opis = str1.replace(&str2, "0");
//         println!("opis are {}",opis);
//         if opis.parse::<i32>().is_ok() {
//             let opi = opis.parse::<i32>().unwrap_or_default();
//             println!("opi is {}",opi);
//             if opi > 0 {
//                 return false
//             } else {
//                 return true;
//             }
//         }else {
//             return false;
//         }

//     }
//     let length = str2.len() as u32;
//     for i in 1..length+1 {
//         if length % i == 0 {
//             let word_length = length/i;
//             let keyword = &str2[0..word_length as usize];
//             println!("keyword {}", keyword);
//             let mut err_flag = false;
//             for j in 1..i {
//                 println!("iterating from {} to {} for word_length {} and number of words {}",
//                 j * word_length, (j * word_length) + word_length, word_length, i);
//                 if &str2[(j * word_length) as usize..((j * word_length) + word_length) as usize] != keyword {
//                     err_flag = true;
//                     break;
//                 }
//             }
//             if err_flag == true {
//                 continue;
//             }else {
//                 if check_in_str1(&str1, keyword.to_string()) == true {
//                     return keyword.to_string();
//                 }else{
//                    continue;
//                 }
//             }
//         }

//     }
//     return "".to_string();

// }
