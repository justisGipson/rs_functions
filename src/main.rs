// basic function declaration
//
// fn main() {
//     println!("Hello, world!");
//     another_function();
// }
//
// fn another_function() {
//     println!("Another function");
// }

// function with parameters
// fn main() {
//     another_function(5);
// }

// fn another_function(x: i32) {
//     println!("The value of x is: {}", x);
// }

// function with multiple params
fn main() {
    another_function(5, 6);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
