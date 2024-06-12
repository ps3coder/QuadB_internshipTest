fn prime_check(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn main() {
    let number = 29;
    println!("Is {} a prime number? {}", number, prime_check(number));
}
// 1) creating the function prime_check and if n modules 2 == 0 or n modules 3 == 0 then return false 
// 2) creating a loop that says that the given number is divisible by only by itself and one or not if the condition found true then it will return true
// 3) now in the main function we will call the function primeCheck()