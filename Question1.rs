fn is_palindrome(s: &str) -> bool {
    let lower_tran = s.to_lowercase();
    
    
    lower_tran.chars().eq(lower_tran.chars().rev())
}

fn main(){
    let string1 = "madam";
    let string2 = "sir";
    
    if is_palindrome(string1){
        println!("'{}' is a palindrome.", string1);
    }else {
                println!("'{}' is not a palindrome.", string1);
    }
    if is_palindrome(string2){
        println!("'{}' is a palindrome.", string2);
    }else {
                println!("'{}' is not a palindrome.", string2);
    }
}




// 1) Defining the function is_palindrome that takes a string as input and return true if its is and false if its not
// 2) now we will convert the string to the lowercase for some exceptional cases.
// 3) Now we will create algo- to check if the reversed string is equal to the original string for palindrome condition