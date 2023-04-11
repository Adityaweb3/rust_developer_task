//Our function  median takes a reference to a slice of i32 numbers as input and returns the median as a f32.
fn median(numbers: &[i32]) -> f32 {
    let len = numbers.len();

    //This code checks if the length of numbers is even or odd. 
    //If it's even, the median is the average of the two middle elements. 
    //The two middle elements are numbers[len / 2 - 1] and numbers[len / 2]. 
    // These are added together and then divided by 2 to give the average. 
    if len % 2 == 0 {
        (numbers[len / 2 - 1] + numbers[len / 2]) as f32 / 2.0

        //If the length of numbers is odd, the median is the middle element. 
        //We find the middle element by taking the integer division of len / 2, 
        //which will give the index of the middle element. 
    } else {
        numbers[len / 2] as f32
    }
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let median_val = median(&numbers);
    println!("Median value: {}", median_val);
}
