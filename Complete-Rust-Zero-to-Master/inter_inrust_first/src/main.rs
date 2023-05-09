fn main() {
    //tuples
    let tuple: (i8, f32, i32) = (5, 3.14, 100);
    println!("{:?}", tuple);
    println!("first value is {:?}", tuple.0);

    //tuples exercise
    let user2 = (30, true, "Jack");
    user_data(user2);

    //arrays
    let mut arr:[&str; 4] = ["Jerry", "George", "Elaine", "Kramer"];
    arr[2] = "Elaine Benis";
    println!("The cast of Seinfeld consists of {:?}", arr);
    println!("The array's total length is {}", arr.len());

    for name in arr.iter()
    {
        println!("{}", name);
    }

    //array exercise
    let mut int_arr = [12, 2, 3, 2, 4, 5];

    for i in 0..6
    {
        if int_arr[i] == 2 
        {
            int_arr[i] = 0;
        }
    }

    for i in 0..6
    {
        println!("{}. {}", i + 1, int_arr[i]);
    }

    //ownership
    let vector1 = vec![2, 4, 6];
    //vector vector1 owns the object in the heap
    //only a single variable owns the heap memory at a given time
    let vector2 = vector1;
    println!("{:?}", vector2);
    //println!("{:?}", vector1);

    //references
    let vector = vec![1, 2, 3];
    display(&vector);
    println!("The second value of our vector is {}", vector[1]);
    //error because vector moved when it is used as argument of the display function.
    
    //reference exercise
    let mut car = String::from("Ferrari");
    display2(&mut car);
    println!("{}", car);

    //slices
    let game = "Mario Brothers".to_string();
    println!("Length of the game is {}", game.len());

    let slice = &game[0..5];
    println!("{}.", slice); //prints Mario

    //exercixe slice
    let mut nums = [1, 2, 3, 4, 5];
    slice_and_dice(&mut nums[1..3]);
    println!("{:?}", nums);

}

fn slice_and_dice(arr:&mut [i32])
{
    println!("The length of the slice is {}", arr.len());
    println!("{:?}", arr);
    arr[0] = 200;
}

fn display2(var: &mut String)
{
    var.push_str(" F8 Tributo");
}

fn user_data(x: (i32, bool, &str))
{
    let (age, active, name) = x;
    println!("{} is {} years old. {}", name, age, active);
}

fn display(x: &Vec<i32>)
{
    println!("{:?}", x);
}