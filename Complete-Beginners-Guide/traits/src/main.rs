#[allow(unused_variables)]
#[allow(dead_code)]
//traits
struct RustDev {
    awesome: bool,
}

struct JavaDev {
    awesome: bool,
}

trait Developer {
    fn new(awesome:bool) -> Self;
    fn language(&self) -> &str;
    fn say_hello(&self) {println!("Hello World!");}
}

impl Developer for RustDev {
    fn new(awesome:bool) -> Self {
       RustDev { awesome:awesome, } 
    }

    fn language(&self) -> &str {
        "Rust"
    }

    fn say_hello(&self) {
        println!("println!(\"Hello World!\")");
    }
}

impl Developer for JavaDev {
    fn new(awesome:bool) -> Self {
        JavaDev { awesome: awesome, }
    }

    fn language(&self) -> &str {
        "Java 1.8"
    }

    fn say_hello(&self) {
        println!("System.out.println(\"Hello World!\")");
    }
}

//generics
trait Bark {
    fn bark(&self) -> String;
}

struct Dog {
    species: &'static str,
}

struct Cat {
    _color: &'static str,
}

impl Bark for Dog {
    fn bark(&self) -> String {
        return format!("{} barking", self.species);
    }
}

fn bark_it<T: Bark>(b: T) {
    println!("{}", b.bark());
}

//returning traits
struct Dog1 {

}

struct Cat1 {

}

trait Animal {
    fn make_noise(&self) -> &'static str;
}

impl Animal for Dog1 {
    fn make_noise(&self) -> &'static str {
        "woof"
    }
}

impl Animal for Cat1 {
    fn make_noise(&self) -> &'static str {
        "miaw"
    }
}

fn get_animal(rand_number: f64) -> Box<dyn Animal> {
    if rand_number < 1.0 {
        Box::new( Dog1{} )
    } else {
        Box::new( Cat1{} )
    }
}

//adding traits to existing structure
trait Summable<T> {
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut sum: i32 = 0;
        for i in self {
            sum += *i;
        }
        sum
    }
}

use std::ops::Add;
//operator overloading
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Self) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}


// static dispatch
trait Duplicatable {
    fn dupl(&self) -> String;
}

impl Duplicatable for String {
    fn dupl(&self) -> String {
        format!("{0}{0}", *self)
    }
}

impl  Duplicatable for i32 {
    fn dupl(&self) -> String {
        format!("{}", *self * 2)
    }
}

fn duplicate<T: Duplicatable>(x: T){
    println!("{}", x.dupl());
}

//dynamic dispatch

fn duplicate1(x: &dyn Duplicatable) {
    println!("{}", x.dupl());
}
fn main() {

    //traits
    let rust = RustDev::new(true);
    let java = JavaDev::new(false);

    println!("rust language is {}({})", rust.language(), rust.awesome);
    println!("java language is {}({})", java.language(), java.awesome);

    rust.say_hello();
    java.say_hello();

    let dog = Dog { species: "retrivier"};
    let cat = Cat { _color: "black"};

    bark_it(dog);
    //bark_it(cat);

    //returning traits
    println!("{}", get_animal(5.5).make_noise());
    println!("{}", get_animal(0.5).make_noise());

    //adding traits to existing structure
    let a = vec![1, 2, 3, 4, 5];
    println!("Sum: {}", a.sum());
    let b = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    //println!("Sum: {}", b.sum()); not implemented for float

    //operator overloading
    let p1 = Point { x: 1.3, y: 4.6};
    let p2 = Point { x: 3.7, y: 1.4};
    let p3 = p1 + p2;

    println!("P: {:?}", p3);

    //static dispatch
    let a = 42;
    let b = "Hi Emre ".to_string();
    
    duplicate(a);
    duplicate(b); 

    //dynamic dispatch

    let c: i32 = 55;
    let d = "Hi Jhon ".to_string();
    duplicate1(&c);
    duplicate1(&d);

}