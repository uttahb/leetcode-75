fn main() {
    println!("Hello, world!");
    let a = vec![1, 2, 3, 4];
    let b = product_except_self(a);
    println!("{:?}", b);
}
fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let o: Vec<i32> = vec![];
    let i = 0;
    let len = nums.len() as i32;
    while i < len {
        i = i + 1;
    }
}
