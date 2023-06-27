//Write a program that prints the numbers from 1 to 100.
//  But for multiples of three, print Fizz instead of the number,
// and multiples of five, print Buzz. For numbers that are multiples of both three and five, print FizzBuzz.

fn main() {
    for i in 1..101 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("fizzbuzz");
        } else if i % 3 == 0 {
            println!("fizz");
        } else if i % 5 == 0 {
            println!("buzz")
        } else {
            println!("{}", i)
        }
    }
}
