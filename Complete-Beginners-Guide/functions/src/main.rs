static mut R: i32 = 0;

macro_rules! my_macro {
    () => (println!("Firts macro"));
}

macro_rules! name {
    ($name: expr) => { println!("Hey {}", $name)}
}

macro_rules! xy {
    (x => $e: expr) => {
        println!("X is {}", $e)
    };
    (y => $e: expr) => {
        println!("Y is {}", $e)
    };
}

macro_rules! name2 {
    ($($name: expr), *) => {
        $(println!("Hey, {}", $name);)*
    };
}

macro_rules! build_fn {
    ($fn_name: ident) => {
        fn $fn_name() {
            println!("{:?} was called", stringify!($fn_name));
        }
    };
}
fn main() {
   {
        let a = 5;
        println!("a: {}", a);
   }

    unsafe { 
        R = 4;
        println!("R: {}", R);
    }

    //closures
    let a = |a: i32| -> i32 {a + 1};
    println!("a: {}", a(5));

    //higher order functions
    let square = |x: i32| -> i32 {x * x};
    apply(square, 5);

    // calculate sum of the all the squares less than 500
    let limit = 500;
    let mut sum = 0;

    for i in 0.. {
        if true == is_even(i) {
            let isq = i * i;
            if isq > limit {
                break;
            } else {
                sum += isq;
            }
        }
    }

    println!("Sum: {}", sum);

    // with HOFs
    let sum2 = (0..).map(|x| x * x )
            .take_while(|&x| x <= limit)
            .filter(|x| is_even(*x))
            .fold(0, |sum, x| sum + x);

    println!("Sum2: {}", sum2);

    //macros
    my_macro!();
    name!("Emre");
    name2!("Emre", "Ayse", "Ahmet", "Mehmet");

    xy!(x => 5);
    xy!(y => 3 * 4);

    build_fn!(hey);
    hey();
}

fn is_even(val: i32) -> bool {
    val % 2 == 0
}

fn apply(f: fn(i32) -> i32, a: i32) {
    println!("Result: {}", f(a));
}
