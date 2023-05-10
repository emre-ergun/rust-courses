use std::fs::File;

fn main() {
    //error handling

    //panic!("This will cause the program to abruptly end"); //panic is causing program to end.
    //let f = File::open("doesnotexist.txt").expect("No such thing!");
    //let f = File::open("doesnotexist.txt").unwrap();
    println!("This program is working here!");

    //exercise for error handling
    let mut solution = is_seven(7).unwrap();
    println!("The solution is {}", solution);
    solution = is_seven(15).unwrap();
    println!("The solution is {}", solution);

}

fn is_seven(val: i32) -> Result<bool, String>
{
    if val == 7
    {
        return Result::Ok(true);
    }
    else {
        return Result::Err("This is not seven".to_string());
    }
}
