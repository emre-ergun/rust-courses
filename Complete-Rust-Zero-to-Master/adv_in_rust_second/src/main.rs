struct GenericStruct<T> {
    value: T,
}

struct Game {
	weapon: &'static str,
	power_level: u32,
}

trait Stats {
	fn character_stats(&self);
}

impl Stats for Game {
	fn character_stats(&self) {
		println!("Printing stats of power level: {}, weapon: {}",
		self.power_level, self.weapon);
	}
}

use std::fs::File;
use std::io::{Error, Read, BufReader}; //buff reader just provides  a buffering comp.
use std::ops::Deref;
struct CustomSmartPointer<T>(T);

impl<T> CustomSmartPointer<T> {
    fn heap_allocation(value:T)->CustomSmartPointer<T> {
        CustomSmartPointer(value)
    }
}

impl<T> Deref for CustomSmartPointer<T> {
    type Target = T; //in traits, type is used to declare an associated type
    fn deref(&self) -> &T {
        &self.0 //syntax for taking the first argument whihc is 0
    }
}
fn main() -> Result<(), Error>{

    let color = "green";
    let ref_color = CustomSmartPointer::heap_allocation(color);
    println!("green is the same as color which is {}", "green"==color);
    println!("green is the same as color which is {}", "green"==*ref_color);

    let greeting = "hello";
    let greeting_heap = Box::new(greeting); //points to a new variable in the heap

    println!("{}", "hello"==greeting); //print true
    println!("{}", "hello"==*greeting_heap); //print true, * is to access the 
									    	// value in the heap

    //closures
    /*
    let closer_function = |parameter| {
        //pass some logic
    } 
    */
    let is_even = |n| {
        n % 2 == 0
    };

    let num = 11;
    println!("{} is even? {}", num, is_even(num));
    //iterations
    let x = [1, 2, 3];
    let iter = x.iter();

    for item in iter {
        print!("{}\t", item);
    }
    println!("");

    //into_iter method moves values in the collection into an inter object via ownership
    let values = vec!["a", "b", "c"];
    for value in values.into_iter() {
        match value {
            "c" => println!("c is a good time"),
            _ => println!("iteration: {}", value),
        }
    }

    //println!("{:?}", values); //error because it is borrowed.

    //exericse iterations
    let mut pets = vec!["cat", "dog", "goldfish"];
    for pet in pets.iter_mut()
    {
        match pet {
            &mut "dog" => println!("cute doggy!"),
            _ => println!("Hello {}!", pet),
        }
    }


    let t1: GenericStruct<i32> = GenericStruct{value:100};
    println!("the value is {}", t1.value);
    let t2: GenericStruct<String> = GenericStruct{value:"String Data Type".to_string(),};
    println!("the value is {}", t2.value);

    //traits example
    let g1 = Game{
        weapon: "Shotgun",
        power_level: 100,
    };

    g1.character_stats();

    //file write
    //write
	let file_name = "cat.txt";
	match File::create(file_name) {
		Ok(file) => println!("Writing {:?}", file),
		Err(_) => println!("Unable to create the file {}", file_name),
	}

    // file open
	match File::open(file_name) {
		Ok(file) => println!("Reading {:?}", file),
		Err(_) => println!("Unable to open the file {}", file_name),
	}

    //file read
	let file = File::open(file_name)?; //? is short-hand form of unpack of error handling
	let mut reader = BufReader::new(file);
	let mut contents = String::new();

	reader.read_to_string(&mut contents)?;
	println!("this is the content: {}", contents);
	Ok(())
}
