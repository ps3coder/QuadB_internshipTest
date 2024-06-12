fn median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len == 0 {
        return 0.0;  
        
    }

    if len % 2 == 0 {

        let mid1 = len / 2;
        let mid2 = mid1 - 1;
        (arr[mid1] as f64 + arr[mid2] as f64) / 2.0
    } else {
        arr[len / 2] as f64
    }
}

fn main() {
    let sorted_array = [1, 2, 3, 4, 5];
    println!("The median is: {}", median(&sorted_array));

}


// 1) we know that Median = (n + 1) ÷ 2
// 2) now we will create a function median that takes the array as a input and if the length is more than 0 then processes to the next step
// 3) so if the length says that the given array is even than we will apply the even method to find median
// 4) and if the number is find odd than we will find the odd method to find the median and return it to the main function
