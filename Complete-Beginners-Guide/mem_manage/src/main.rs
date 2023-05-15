#[derive(Debug)]
struct Person {
    name: String,
}

#[derive(Debug)]
struct Dog<'l> {
    name: String,
    owner: &'l Person,
}

impl Person {
    fn get_name(&self) -> &String {
        &self.name
    }
}

//reference counted variables
use std::rc::Rc;

struct Car {
    brand: Rc<String>,
}

impl Car {
    fn new(brand:Rc<String>) -> Car {
        Car {
            brand: brand,
        }
    }

    fn drive(&self) {
        println!("{} is driving", &self.brand);
    }
}
fn main() {

    //ownership
    let i = 5;
    let j = i; //data copied

    println!("{}", j);
    println!("{}", i);
    
     let v = vec![1, 2, 3, 4, 5];
    // let w = v; // ownership transferred
    // println!("{:?}", w);
    //println!("{:?}", v); // err: value borrowed

    //ownership is given to function then it is returned back to
    //assigned value
    let foo = |v: Vec<i32>| -> Vec<i32> {
        println!("Vector used in foo");
        v
    };

    let x = foo(v);
    println!("{:?}", x);

    //borrowing
    let mut a = 6;
    let b = &mut a;
    println!("{}", *b);
    *b += 2;
    println!("{}", a);


    let mut vec1 = vec![1, 2, 3, 4, 5];

   for i in &vec1 {
        println!("{}", i);
        //vec1.push(6); //error because it is borrowed first as mutable
   } 
   vec1.push(6); //no error, because it is ownership trasnferred back
                 // after loop

    //lifetimes

    println!("{}", get_str()); //static lifetime ends if program ends

    let p1 = Person {name: String::from("Emre")};
    let d1 = Dog { name: String::from("Max"), owner: &p1};

    println!("{:?}", d1);

    let mut s:&String;
    {
        let p2 = Person { name: String::from("Marry")};
        //s = p2.get_name();
        s = p1.get_name();
    }

    println!("{}", s);

    //reference counted variables
    let brand = Rc::new(String::from("BMW"));
    println!("Pointers: {}", Rc::strong_count(&brand));
    {
        let car = Car::new(brand.clone());
        car.drive();
        println!("Pointers: {}", Rc::strong_count(&brand));
    }
    println!("My car is {}", brand);
    println!("Pointers:{}", Rc::strong_count(&brand));
    
}

//lifetimes

fn get_str() -> &'static str {
    "hello"
}
