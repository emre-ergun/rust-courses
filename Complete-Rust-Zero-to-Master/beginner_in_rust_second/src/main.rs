fn main() {
    let mut greeting = String::from("Julia says, ");
    
    greeting.push_str("hello!");
    println!("{}", greeting);

    //convert a string literal into a String Object
    let random_str = "Please make me into an object!".to_string();
    println!("{}", random_str);

    let mut password = "pokemon,".to_string();
    password.push_str(" gotta cath them all");
    println!("{}", password);

    //statements
    let user = "todd";

    let a_str = if user.len() == 4
    {
        "Length is four bytes"
    }
    else
    {
        ""
    };

    println!("{}", a_str);
    let mut b_str = a_str.to_string();
    b_str.push_str(" hello");
    println!("{}", b_str);

    let microbiome = "xc12";
    let body_part = match microbiome {
            "xc12" => { println!("Found match for microbiome!"); "Tummy Biome"},
            "mpt1" => "Eye Biome",
            "ttw6" => "Finger Biome",
            _ => "Unknown"
    };

    println!("The biome match is {}", body_part);

    //exercise
    let mut count: u32 = 0;

    loop {
        count += 1;

        if count == 3
        {
            println!("Welcome to Miami!");
        }
        else if count == 5
        {
            println!("Time to call it a day!");
            break;
        }
    }

    //exercise function
    let y = plus_one(5);
    println!("y is {}", y);


}

fn plus_one(x: i32) -> i32
{
    return x + 1;
}