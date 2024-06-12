fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() {
    let original = "ps3coder";
    let reversed = reverse(original);
    println!("Original String is: {}", original);
    println!("Reversed String is: {}", reversed);
}
// 1) lets create a reverse function that use the rev() method to reverse the string
// 2) now we will assign the function in the main function 