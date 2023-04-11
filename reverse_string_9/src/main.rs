// The reverse_string function takes a string  s as input 
//and returns a new String with its characters reversed.



fn reverse_string(s: &str) -> String {

    // Here firstly we are calling chars to find an iterator
    // Then calling rev method on this iterator to reverse the order of the values
    // Then collect method to convert the reversed iterator into a new String
    s.chars().rev().collect()
}

fn main() {
    let s = "hello world";
    let reversed = reverse_string(s);
    println!("Reversed string: {}", reversed);
}
