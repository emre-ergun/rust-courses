//how to write comments in Rust

/*
    Comments

    Any program requires comments, and Rust supports a few different varieties:
    
    Regular comments which are ignored by the compiler:

    "// : double line comments which go to the end of a line"
    "/* */ : Block comments go to the closing delimeter"
    "/// : doc comments are parsed into HTML library documentation"
    "//! : generate library documents for a following itemn (enclosing item //!)"
 */

fn main(){
// hello

/*
    hello there
    this is a block comment
 */
    println!("Hello 🌎!");

    let vec1: Vec<i32> = vec![3; 5];
    println!("{}", vec1[4]);

    let financial_management = "Bank of America"; //string type
    let credit_score = 800; //integer type (are wwhole numbers no decimals)
    //float types take decimal
    let account_active = true; //boolean type
    
    println!("My current investior portfolio is managed by: {}", financial_management);
    println!("My credit score is: {}", credit_score);
    println!("Account active: {}", account_active);

    let switch = false;
    let volume = 10;

    println!("Can I switch? {}", switch);
    println!("The volume is {}", volume);

    let a_string = format!("hey {}", volume);
    println!("{}", a_string);
    /*
        format!
        print! io::stdout
        println!
        eprint! io::stderr
        eprintln!
     */

    println!("{} years old", 34);
    //named arguments also work for println macro
    println!("{user1} {action} {user2}", user1="Emre", user2="Ergun", action=35);

    let total:u32 = 4; //defulat integer will be i32
    let height: u32 = 41;
    let deduction: i32 = 2 - 200;

    println!("the total is {}", total);
    println!("the height is {}", height);
    println!("the deduction is {}", deduction);

    let mut x = 5;
    println!("{}", x);
    x = x + 1;
    println!("{}", x);
    const SHOULD_BE_UPPER: i32 = 6;
    println!("{}", SHOULD_BE_UPPER);
    const STRING: &str = "Hello World";
    println!("{}", STRING);

    let greeting = "Hi Emre";
    println!("{}", greeting);

    let nothing_within = String::new();
    println!("Length of the string is {}", nothing_within.len());

    let great_movie = String::from("The Big Lebovski");
    println!("{} is a great movie.{}", great_movie, great_movie.len());
    print_movie(&great_movie);
    println!("{} is a great movie.{}", great_movie, great_movie.len());

    let value1 = 35;
    print_value(value1);
    println!("Value is {}", value1);


}

fn print_movie(movie: &String)
{
    println!("{} is a great movie", movie);
}
fn print_value(value: i32)
{
    println!("{} is a great value", value);
}