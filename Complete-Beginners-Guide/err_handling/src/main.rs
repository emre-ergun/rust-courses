use std::fs::{File, OpenOptions, remove_file};
use std::io::{Write, Read};
use std::io;

fn main() {
    //file operations
    let mut file = File::create("src/test.txt").expect("create failed");
    
    file.write_all("Hello, World!\n".as_bytes()).expect("write failed");

    let mut file1 = OpenOptions::new().append(true)
                            .open("src/test.txt")
                            .expect("open failed");
    file1.write_all("World, Hello!\n".as_bytes()).expect("failed");

    let mut file2 = File::open("src/test.txt").unwrap();
    let mut contents= String::new();
    file2.read_to_string(&mut contents).unwrap();
    println!("Content:\n{}", contents);

    remove_file("src/test.txt").expect("remove failed");  

    //errors      
    // let v = vec![1, 2, 3, 4, 5];
    // v[10]; //main thread panic
    //panic!("Something went wrong! can not proceed!");

    let f = File::open("main.jpg");
    match f {
        Ok(f) => {
            println!("file found {:?}", f);
        },
        Err(e) => {
            println!("file not found {:?}", e);
        }
    }

    println!("Continuing on with execution");

    divide(Some(5));
    divide(None);
    //divide(Some(0));

    //helper method
    //let f1 = File::open("main.jpg").unwrap();
    //let f3 = File::open("main.jpg").expect("failed to open");


    let a = read_username_from_file();
    println!("{:?}", a);
    let b = read_username_from_file1();
    println!("{:?}", b);
}
// \? operator
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("src/username.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file1() -> Result<String, io::Error>{
    let mut f = File::open("src/username.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
//errors
const ANSWER_TO_LIFE: i32 = 42;
fn divide(x: Option<i32>) {
    match x {
        Some(0) => panic!("Can not divide by zero"),
        Some(x) => println!("result is {}", ANSWER_TO_LIFE / x),
        None => println!("None received, answer is {}", ANSWER_TO_LIFE)
    }
}