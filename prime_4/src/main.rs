// In this function we are taking a prime number and return bool wheather  this number is prime or not .

fn is_prime(n: u64) -> bool {

    //This line checks if n is less than 2, which is not a prime number by definition. 
    // If n is less than 2, the function immediately returns false.
    if n < 2 {
        return false;
    }
    //iteration from 2 up to the square root of n. 
    // The loop checks whether any of these numbers evenly divide n and, if so, returns false
    for i in 2..=(n as f64).sqrt() as u64 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let n1 = 39;
    let n2 = 17;
    let n3 = 47;
    println!("{} is prime: {}", n1, is_prime(n1));
    println!("{} is prime: {}", n2, is_prime(n2));
    println!("{} is prime: {}", n3, is_prime(n3));
}