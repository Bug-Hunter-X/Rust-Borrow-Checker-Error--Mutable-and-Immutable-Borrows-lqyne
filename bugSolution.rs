fn main() {
    let mut x = 5;
    { // Create a new scope for the mutable borrow
        let y = &mut x; // y is a mutable reference to x
        *y += 1; // Modify x through y
    }

    let z = &x; // z is an immutable reference to x
    println!("x = {}", x); // x is 6
    println!("z = {}", *z); // This is now allowed 
} 