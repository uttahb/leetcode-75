fn main() {
    println!("Hello, world!");
    let s = vec![1, 2, 3, 4, 5, 25, 24, 3, 4];
    println!("{}", max_area(s));
}
fn max_area(height: Vec<i32>) -> i32 {
    let mut max_area = 0;
    let len = height.len() as i32;
    let mut left = 0;
    let mut right = len - 1;
    // for i in 0..len {
    //     let mut k = len - 1;
    //     for j in (i + 1)..len {
    //         let mut min = height[i as usize];
    //         if min > height[j as usize] {
    //             min = height[j as usize];
    //         }
    //         let areaj = (j - i) * min;
    //         // println!("areaj = {} - {} * {} = {}", j, i, min, areaj);
    //         min = height[i as usize];
    //         if min > height[k as usize] {
    //             min = height[k as usize];
    //         }
    //         let areak = (k - i) * min;
    //         // println!("areak = {} - {} * {} = {}", k, i, min, areak);
    //         // println!("i = {}, j={}, k={}", i, j, k);
    //         // println!(
    //         //     "i = {}, j = {}, k ={} h[i] = {}, h[j] ={}, h[k] ={}, areaj = {}, areak={}",
    //         //     i, j, k, height[i as usize], height[j as usize], height[k as usize], areaj, areak
    //         // );
    //         if max_area < areaj {
    //             max_area = areaj;
    //         }
    //         if max_area < areak {
    //             max_area = areak;
    //         }
    //         k -= 1;

    //         if j > k {
    //             break;
    //         }
    //     }
    // }
    // return max_area;

    // time limit exceeded
    // community solution

    while len > 0 {
        let area = height[right as usize].min(height[left as usize]) * (right - left);
        if max_area < area {
            max_area = area;
        }
        if height[left as usize] < height[right as usize] {
            left += 1;
        } else {
            right -= 1;
        }
        if right < left {
            break;
        }
    }
    // here we are shrinking the conainer from right and left together
    // each iteration increments either right or left index based on which arm is smaller
    // i had a doubt  what makes this work, since even if its a smaller arm
    // if distance between the arms are long enough, could that add to area? shouldn't we compare each arm with
    // every other arm?
    // but its not like that. even if we advance from the longer arm side, the area won't increase in any way,
    // max height to which water can container is still that min height.
    max_area
}
