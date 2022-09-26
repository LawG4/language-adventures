// Import the function from the example library that we made
use library::library_example_add;

fn main() {

    // Diplay this sum to the user 
    let x = 4;
    let y = 3;
    let sum_from_lib = library_example_add(x, y);

    println!("The value of {x} + {y} = {sum_from_lib}")
}
