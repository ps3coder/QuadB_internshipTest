fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    if left < arr.len() && arr[left] == target {
        Some(left)
    } else {
        None
    }
}

fn main() {
    let sorted_array = [10, 20, 20, 20, 30,30 ,40, 50];
    let target = 30;

    match find_first_occurrence(&sorted_array, target) {
        Some(index) => println!("The first occurrence of {} is at index {}.", target, index),
        None => println!("The target {} is not in the array.", target),
    }
}


//  1) first we need to create a function find_first_occ and then applying the binary search logic which ensure that we narrow down the range to the first occurance of the given target which we will give later
// 2) now we are creating a main function and defining the array 
// 3) now assiging the target for example 30
// 4) and then assigning the find_first_occurrence function .
