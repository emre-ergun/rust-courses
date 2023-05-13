#[allow(unused_variables)]
fn main() {
    //arrays
    let primes = [2, 3, 5, 7, 11];
    let doubles: [f64; 4] = [2.0, 4.0, 6.0, 8.0];

    println!("primes: {:?}", primes);
    println!("doubles: {:?}", doubles);
    
    //vectors
    let mut primes: Vec<i32> = Vec::new();
    let mut doubles = vec![2, 4, 6]; //or
    let mut vector1 = vec![2; 5]; // five items, each item is two

    primes.push(13);
    doubles.push(8);
    vector1.remove(2);
    vector1[0] = 0;

    println!("primes: {:?}", doubles);
    println!("doubles: {:?}", doubles);
    println!("vector1: {:?}", vector1);

    for number in doubles.iter() {
        print!("{} ", number * number);
    }
    println!();

    //slices
    let values = [1, 2, 3, 4, 5];
    let slice = &values[1..4]; // 2, 3, 4
    println!("{:?}", slice);

    let mut colors = ["red", "green", "blue", "yellow", "black"];
    println!("Colors: {:?}", colors);
    
    update_colors(&mut colors[3..=4]);
    println!("Updated Colors: {:?}", colors);

    //struct    
    let emre = Employee {
        name: String::from("Emre"),
        company: String::from("Engramsoft"),
        age: 34
    };

    println!("{} {}", Employee::fn_static_details(), emre.fn_details());

    //enums
    let my_color1 = Colors::Red;
    let my_color2 = Colors::Green;
    let my_color3 = Colors::Blue;

    let a_person = Person::Name(String::from("Emre"));
    println!("person: {:?}", a_person);

    //generics
    let point1: Point<i32> = Point {x:6, y:8};
    let point2: Point<f64> = Point {x:6.25, y:8.75};

    println!("p1: {:?}, p1: {:?}", point1, point2);

}
// generics
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T
}
//enums
enum Colors {
	Red,
	Green,
	Blue
}

#[derive(Debug)]
enum Person {
	Name(String),
	Surname(String),
	Age(u32)
}

//structs
struct Employee {
    name: String,
    company: String,
    age: u32
}

impl Employee {
    fn fn_details(&self) -> String {
        format!("{},{},{}", self.name, self.company, self.age)
    }

    fn fn_static_details() -> String {
        format!("Details are: ")
    }
}

//slices
fn update_colors(color_slice: &mut [&str]) {
    for color in color_slice.iter_mut() {
        *color = "red";
    }
}
