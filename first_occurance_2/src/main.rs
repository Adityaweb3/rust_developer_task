// Basically we are using Binary search algorithm here for finding first occurance of target value 


fn first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    let mut l = 0;
    let mut r = arr.len() - 1;

    while l <= r {
        let mid = (l + r) / 2;

        if arr[mid] == target {
            
            let mut i = mid;
            while i > 0 && arr[i - 1] == target {
                i -= 1;
            }
            return Some(i);
           // If the value at mid is less than the target,
           // the function sets left to mid + 1 to search the right half of the array. 
           //If the value at mid is greater than the target, the function sets right to mid - 1 to search the left half of the array. 
           //The function repeats this process until it finds the first occurrence of the target or determines that the target does not occur in the array.
           
        } else if arr[mid] < target {

            
            l = mid + 1;
        } else {
            
            r = mid - 1;
        }
    }

    .
    None
}

fn main() {
    let arr = [2,2,8,5,9,7,7];
    let target = 7;
    match first_occurrence(&arr, target) {
        Some(index) => println!("The first occurrence of {} is at index {}", target, index),
        None => println!("{} does not occur in the array", target),
    }
}

