use rand::Rng;

fn main() {

    //if statement
    let mut rng = rand::thread_rng();
    let num: u8= rng.gen_range(0, 11);

    if num < 5 {
        println!("The random number {} is lesser than 5", num);
    } else if num == 5 {
        println!("The random number {} is equal to 5", num);
    } else {
        println!("The random number {} is bigger than 5", num);
    }

    let res: bool = if num > 5 { true } else { false };
    println!("Result: {}", res);
    
    //match statement
    print_choice(Suit::Heart);
    print_choice(Suit::Spade);
    print_choice(Suit::Club);
    print_choice(Suit::Diamond);

    country(44);
    country(34);
    country(55);
    country(1235);


}

fn country(code: i32) {
    let country = match code {
        44 => "UK",
        34 => "Spain",
        1..=999 => "unknown",
        _ => "invalid" 
    };

    println!("The country is {}", country);
}
enum Suit {
    Heart,
    Spade,
    Club,
    Diamond
}

fn print_choice(choice: Suit) {
    match choice {
        Suit::Heart => {
            println!("\u{2665}");
        },
        Suit::Spade => {
            println!("\u{2660}");
        },
        Suit::Club => {
            println!("\u{2663}");   
        },
        Suit::Diamond => {
            println!("\u{2666}");
        }
    }
}