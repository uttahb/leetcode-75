fn main() {
    let candies = vec![2, 3, 5, 1, 3];
    let extra_candies = 3;
    let op = kids_with_candies(candies, extra_candies);
    println!("{:?}", op);
}
pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let max = &candies.iter().max().unwrap();
    let mut kids: Vec<bool> = vec![];
    for kid in &candies {
        println!("comparing max {} with kid+extra {}", *max, kid + extra_candies);
        if **max >  kid + extra_candies {
            kids.push(false);
        }else {
            kids.push(true);
        }
    }
    return kids;
}
