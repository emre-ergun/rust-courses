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
}

fn user_data(x: (i32, bool, &str))
{
    let (age, active, name) = x;
    println!("{} is {} years old. {}", name, age, active);
}