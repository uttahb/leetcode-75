fn main() {
    let mut flowerbed = vec![1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0];
    let mut n = 3;
    let can_place = can_place_flowers(flowerbed, n);
    println!("so can we place?  {}", can_place);
}
// fn can_place_flowers(flowerbeds: Vec<i32>, m: i32) -> bool {
//     let mut n = m;
//     let mut flowerbed = flowerbeds.clone();
//     let flowerbed_len = flowerbed.len();
//     for (i, item) in flowerbeds.iter().enumerate() {
//         if n == 0 {
//             return true;
//         }
//         if flowerbed_len == 1 && *item == 0 {
//             return true;
//         } else if flowerbed_len == 1 {
//             return false;
//         }
//         if *item == 0 as i32
//             && i > 0
//             && i < flowerbed_len - 1
//             && flowerbed[i - 1] == 0
//             && flowerbed[i + 1] == 0
//         {
//             flowerbed[i] = 1;
//             n = n - 1;
//         } else if *item == 0 && i == 0 && flowerbed[i + 1] == 0 {
//             flowerbed[i] = 1;
//             n = n - 1;
//         } else if *item == 0 && i == flowerbed_len - 1 && flowerbed[i - 1] == 0 {
//             flowerbed[i] = 1;
//             n = n - 1;
//         }
//     }
//     n == 0
// }

// community code
fn can_place_flowers(mut flowerbed: Vec<i32>, n: i32) -> bool {
    println!("{:?}", flowerbed);
    flowerbed.push(0);
    flowerbed.insert(0, 0);
    let newn = flowerbed
        .split(|&k| k == 1)
        .map(|v| {
            println!("splits are {:?}", v);
            return (v.len() - 1) / 2;
        })
        .map(|x| {
            println!("{:?}", x);
            return x;
        })
        .sum::<usize>();
    println!("sum is {}", newn);
    newn >= n as usize
}
