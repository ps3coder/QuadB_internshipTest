

fn main() {
    let mut arr1:[i32;5] = [0,1,2,3,4];
    let mut arr2:[i32;5] = [5,6,7,8,9];
    let mut arr3:[i32;10] = [0;10];
    
    let mut i:usize = 0;
    let mut j:usize = 0;
    
    while i<5
    {
        arr3[i]=arr1[i];
        i = i + 1;
    }
    
    while j<5
    {
        arr3[i]=arr2[j];
        i = i + 1;
        j = j + 1;
    }
    
    println!("arr1: {:?}",arr1);
    println!("arr2: {:?}",arr2);
    println!("arr3: {:?}",arr3);
}
// Here, we created two arrays of the integers with 5 elements and 3rd array with 10 elements initialized with 0. 
//Then we copy the elements of arr1 and arr2 into arr3.
// After that, we printed the all arrays.


