fn main() {
    println!("Hello, world!");
    let a = vec![1, 2, 3, 4];
    let b = product_except_self(a);
    println!("{:?}", b);
}
fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    let mut o: Vec<i32> = vec![1; len as usize];
    let mut r = 1;
    let mut l = 1;
    let mut j = 0;
    let mut k = len - 1;
    loop {
        println!("k is {} and j is {}", k, j);
        o[j] = l * o[j];
        o[k] = o[k] * r;
        l *= nums[j];
        r *= nums[len - j - 1];
        if k == 0 {
            break;
        }
        j += 1;
        k -= 1;
    }
    return o;
}
