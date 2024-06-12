//Code explanation

1.
Implement a function that checks whether a given string is a palindrome or not.
// 1) Defining the function is_palindrome that takes a string as input and return true if its is and false if its not
// 2) now we will convert the string to the lowercase for some exceptional cases.
// 3) Now we will create algo- to check if the reversed string is equal to the original string for palindrome condition


2.
Given a sorted array of integers, implement a function that returns the index of the first occurrence of a given number.
//  1) first we need to create a function find_first_occ and then applying the binary search logic which ensure that we narrow down the range to the first occurance of the given target which we will give later
// 2) now we are creating a main function and defining the array 
// 3) now assiging the target for example 30
// 4) and then assigning the find_first_occurrence function .




3.
Given a string of words, implement a function that returns the shortest word in the string.
// 1)We use the min_by_key method to find the word with the minimum length
// 2)The function returns an Option<&str>. If the string is empty or contains no words, it returns None.
// 3) In the main function, we handle the Option and print the shortest word if it exists.
// 4) We use the split_whitespace method to split the string into words based on whitespace.




4.
Implement a function that checks whether a given number is prime or not.
// 1) creating the function prime_check and if n modules 2 == 0 or n modules 3 == 0 then return false 
// 2) creating a loop that says that the given number is divisible by only by itself and one or not if the condition found true then it will return true
// 3) now in the main function we will call the function primeCheck()




5.
Given a sorted array of integers, implement a function that returns the median of the array.
// 1) we know that Median = (n + 1) ÷ 2
// 2) now we will create a function median that takes the array as a input and if the length is more than 0 then processes to the next step
// 3) so if the length says that the given array is even than we will apply the even method to find median
// 4) and if the number is find odd than we will find the odd method to find the median and return it to the main function





6.
Implement a function that finds the longest common prefix of a given set of strings.
// 1) creating the longest_comman_prefix function and check if the slice of array is empty or not
// 2) now we will first start with the string in the slice as the initial prefix to check the comman string in all the word or string
// 3) now if the string starts with the current prefix if its false then we will shorting the prefix and move to the next condition in another string
// 4) and then in the main function we will assign the function




7)Implement a function that returns the kth smallest element in a given array.




8.
Given a binary tree, implement a function that returns the maximum depth of the tree.



9.
Reverse a string in Rust
// 1) lets create a reverse function that use the rev() method to reverse the string
// 2) now we will assign the function in the main function 


10.
Check if a number is prime in Rust
// 1) creating the function prime_check and if n modules 2 == 0 or n modules 3 == 0 then return false 
// 2) creating a loop that says that the given number is divisible by only by itself and one or not if the condition found true then it will return true
// 3) now in the main function we will call the function primeCheck()

11.
Merge two sorted arrays in Rust
// Here, we created two arrays of the integers with 5 elements and 3rd array with 10 elements initialized with 0. 
//Then we copy the elements of arr1 and arr2 into arr3.
// After that, we printed the all arrays.




12.
Find the maximum subarray sum in Rust
// max_subarray_sum function takes a slice of integers of array and return maximum sum of any contiguous subarray
// and now we will initiallized two variable "max_sum" and "current_sum" 
// and then we will assign this function to the main function
