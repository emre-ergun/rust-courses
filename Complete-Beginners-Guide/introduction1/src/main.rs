use std::io;

/// Crate comment.  
/// What is this module trying to achieve?
fn main() {
    //! # Main function
    //! 
    //! ```
    //! fn main()
    //! ````
    //! 
    //! 
    //! Reads user inputs and prints to the console
    //! 
    let mut input = String::new();

    println!("Say something!");

    match io::stdin().read_line(&mut input){
        Ok(_) => {
            println!("You said: {}", input);
        },
        Err(e) => {
            println!("Something went wrong: {}", e);
        }
    }

    print_hello();
}

/// Explanation for print_hello function
/// 
fn print_hello() {
    //! # Print "hello" function
    //! 
    //! ````
    //! fn print_hello()
    //! ````
    //!
    println!("Hello");
}
