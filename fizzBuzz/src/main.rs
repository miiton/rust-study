fn main() {
    for i in 1..100 {
        if i%3 == 0 {
            println!("{} {}", i, "FizzBuzz!!")
        } else if i%2 == 0 {
            println!("{} {}", i, "Buzz")
        } else {
            println!("{} {}", i, "Fizz")
        }
    }
}
