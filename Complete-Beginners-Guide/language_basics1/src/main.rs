use std::println;

const URL: &str = "google.com";

fn main() {
    let name = "Emre"; //type as default is &str
    let age = 34; //type as default is i32

    //explicitly
    let value: u32 = 46; //type is specified explicitly as u32
    
    let cat: &'static str = "Fluffy";

    println!("{}", cat);

    let mut bird = String::from("Tweety");

    let owner = format!("Hi I am {} the owner of {}", "Emre", bird);
    println!("{}", owner);

    let length = owner.len();
    println!("{}", length);

    bird.push(' ');
    bird.push_str("the bird");
    println!("{} {}", bird, bird.len());

    let new_bird = bird.replace("the", "my");
    println!("{}", new_bird);

    println!("{}", URL);

    //pass by value
    let mut name = "Emre";
    //call by value
    say_hello(name);

    //call by reference
    say_hi(&mut name);

    //value is changed because of reference
    println!("{}", name);

    let mut a = 5;
    let mut b = 16;

    swap(&mut a, &mut b);
    println!("a:{}, b:{}", a, b);

}

fn say_hello(name: &str) {
    println!("Hello, {}", name);
}

fn say_hi(name: &mut &str) {
    *name = "Alex";
    println!("Hello, {}", name);
}

fn swap(val1:&mut i32, val2: &mut i32) {
    let temp = *val1;
    *val1 = *val2;
    *val2 = temp;
}