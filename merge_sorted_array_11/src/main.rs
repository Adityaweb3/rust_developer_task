fn merge_sorted(v1: &[i32], v2: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(v1.len() + v2.len());
    let mut i = 0;
    let mut j = 0;
    while i < v1.len() && j < v2.len() {
        if v1[i] < v2[j] {
            result.push(v1[i]);
            i += 1;
        } else {
            result.push(v2[j]);
            j += 1;
        }
    }
    result.extend_from_slice(&v1[i..]);
    result.extend_from_slice(&v2[j..]);
    result
}

fn main() {
    let arr1 = vec![1, 3, 5];
    let arr2 = vec![2, 4, 6];
    let merged_arr = merge_sorted(&arr1, &arr2);
    println!("Merged array: {:?}", merged_arr);
}
