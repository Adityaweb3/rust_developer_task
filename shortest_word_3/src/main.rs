//here we are importing the min function from the cmp module of the std crate rust library
use std::cmp::min;


// Here we are creating a function  shortest_word function that takes a string  as input and returns a string.

fn shortest_word(s: &str) -> &str {
    let mut shortest = s;
    for word in s.split_whitespace() {

        //here we are  comparing the current value of shortest with the word variable, 
        //which represents the current word in the loop. The min function returns the minimum value of its two arguments, 
        //so if word is shorter than shortest, shortest is updated to word
        shortest = min(shortest, word);
    }
    shortest
}

fn main() {
    let s = "To be or not to be, that is the question";
    let shortest = shortest_word(s);
    println!("Shortest word: {}", shortest);
}