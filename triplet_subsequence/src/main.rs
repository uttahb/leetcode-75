fn main() {
    println!("Hello, world!");
    let nums = vec![2,4,-2,-3];
    println!("{}", increasing_triplet(nums));
}
// exceeded time limit
// pub fn increasing_triplet(nums: Vec<i32>) -> bool {
//     let len = nums.len();
//     for i in (0..len).rev() {
//         for j in (0..i).rev() {
//             if nums[i] > nums[j] {
//                 for k in (0..j).rev() {
//                     if nums[k] < nums[j] 
//                         return true;
//                     }
//                 }
//             }
//         }
//     }
//     return false;
// }

// community solution
pub fn increasing_triplet(nums: Vec<i32>) -> bool {
    let max = nums.iter().max();
    let mut first = match max {
        Some(max) => *max,
        None      => std::i32::MAX,
    };
   let mut second = first;
   for i in nums {
    if i <= first {
        first = i;
    }else if i <= second {
        second = i;
    } else {
        return true;
    }
   }
   return false;
 }

