use rand::prelude::*;

fn partition(arr: &mut [i32], low: usize, high: usize) -> usize {
    let pivot_index = low + thread_rng().gen_range(0..=high-low);
    arr.swap(pivot_index, high);
    let pivot = arr[high];
    let mut i = low;

    for j in low..high {
        if arr[j] < pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, high);
    i
}

fn quickselect(arr: &mut [i32], low: usize, high: usize, k: usize) -> i32 {
    if low == high {
        return arr[low];
    }

    let pivot_index = partition(arr, low, high);

    if k == pivot_index {
        arr[k]
    } else if k < pivot_index {
        quickselect(arr, low, pivot_index - 1, k)
    } else {
        quickselect(arr, pivot_index + 1, high, k)
    }
}

fn kth_smallest(arr: &mut [i32], k: usize) -> Option<i32> {
    if k == 0 || k > arr.len() {
        return None;
    }
    Some(quickselect(arr, 0, arr.len() - 1, k - 1))
}

fn main() {
    let mut array = [3, 2, 1, 5, 4];
    let k = 3;
    match kth_smallest(&mut array, k) {
        Some(value) => println!("The {}-th smallest element is {}", k, value),
        None => println!("Invalid input"),
    }
}
