use std::ops::Rem;

fn main() {
    println!("Hello, world!");
    let nums = vec![1, 2, 3, 4];
    println!("{}", max_operations(nums, 5));
}
fn max_operations(mut nums: Vec<i32>, k: i32) -> i32 {
    let mut ops = 0;
    let mut i = 0;
    let mut len = nums.len();
    loop {
        println!("i value is {}", i);
        let mut j = i + 1;
        println!("nums {:?}", nums);
        let mut is_array_changed = false;
        while j < len {
            if nums[i] + nums[j] == k {
                println!("i = {} and j = {}", i, j);
                let _ = nums.remove(j);
                let _ = nums.remove(i);
                len -= 2;
                i = 0;
                is_array_changed = true;

                ops += 1
            }

            j += 1;
        }
        println!("nums {:?}, i is {}", nums, i);
        if i == len {
            println!("len values is {} and i is = {}", len, i);
            return ops;
        }
        if !is_array_changed {
            i += 1;
        }
    }
}
