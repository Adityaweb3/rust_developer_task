// This  function takes a string  s as input and returns a boolean value
// If it returns true then string is palindrome and if false then string is not palindrome

// This code has O(n) time complexity and O(1) space complexity

fn is_palindrome(s: &str) -> bool {

    // Considering two mutable variables l and r these two shows the starting index and last index of string 
    let mut l = 0;
    let mut r = s.len() - 1;


    // In this loop we are comparing wheather left index (l) and right index (r) of string is same or not 
    // We can also use for loop in place of while loop 
    while l < r {

        // as_bytes() method helps us in getting bytes representation of our string for comparision of left and right index 
        // we can also use chars() in place of as_bytes here 
        if s.as_bytes()[l] != s.as_bytes()[r] {
            return false;
        }
        l += 1;
        r -= 1;
    }

    true
}

fn main() {
    let input_string = String::from("Aditya"); 
    let result = is_palindrome(&input_string);

    if result {
        println!("'{}' is a palindrome", input_string);
    } else {
        println!("'{}' is not a palindrome", input_string);
    }
}
