fn longest_common_prefix(strs: &[String]) -> String {
    if strs.is_empty() {
        return String::new();
    }

    let mut prefix = &strs[0][..];
    
    for s in strs.iter().skip(1) {
        while !s.starts_with(prefix) {
            if prefix.is_empty() {
                return String::new();
            }
            prefix = &prefix[0..prefix.len() - 1];
        }
    }

    prefix.to_string()
}

fn main() {
    let strings = vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ];
    println!("The longest common prefix is: {}", longest_common_prefix(&strings));

   
}


// 1) creating the longest_comman_prefix function and check if the slice of array is empty or not
// 2) now we will first start with the string in the slice as the initial prefix to check the comman string in all the word or string
// 3) now if the string starts with the current prefix if its false then we will shorting the prefix and move to the next condition in another string
// 4) and then in the main function we will assign the function
