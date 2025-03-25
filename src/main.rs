
use ndarray::array;
fn main() {
    // Define matrices
    let o = array![[0, 1, 0, 0]];
    let l = array![[0, 0, 1, 0]];
    let r = array![[0, 0, 0, 1]];

    // Witness vector
    let a = array![1, 4223, 41, 103];

    // Compute left and right sides of the equation
    let left = o.dot(&a);
    let right = l.dot(&a) * r.dot(&a);

    // Check if the constraint holds
    let constraint_holds = left == right;

    // Ensure equality holds
    assert!(constraint_holds, "Result contains an inequality");

    println!("R1CS constraint verified successfully!");
}
