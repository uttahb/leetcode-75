use std::ops::Rem;

fn main() {
    println!("Hello, world!");
    let nums = vec![3, 5, 1, 5];
    println!("{}", max_operations(nums, 2));
}
fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums_clone = nums.clone();
    nums_clone.sort();
    let nums_sorted = nums_clone
        .into_iter()
        .filter(|x| x < &k)
        .collect::<Vec<i32>>();
    if nums_sorted.len() == 0 {
        return 0;
    }
    let mut ops = 0;
    let mut i = 0;
    let mut len = nums_sorted.len() - 1;

    loop {
        if len <= i {
            return ops;
        }
        let sum = nums_sorted[i] + nums_sorted[len];

        if sum > k {
            len -= 1;
        } else if sum < k {
            i += 1;
        } else {
            len -= 1;
            i += 1;
            ops += 1;
        }
    }
    // my solution - output time limit
    // loop {
    //     println!("i value is {}", i);
    //     let mut j = i + 1;
    //     println!("nums {:?}", nums);
    //     let mut is_array_changed = false;
    //     while j < len {
    //         if nums[i] + nums[j] == k {
    //             println!("i = {} and j = {}", i, j);
    //             let _ = nums.remove(j);
    //             let _ = nums.remove(i);
    //             len -= 2;
    //             i = 0;
    //             is_array_changed = true;

    //             ops += 1
    //         }

    //         j += 1;
    //     }
    //     println!("nums {:?}, i is {}", nums, i);
    //     if i == len {
    //         println!("len values is {} and i is = {}", len, i);
    //         return ops;
    //     }
    //     if !is_array_changed {
    //         i += 1;
    //     }
    // }
}
