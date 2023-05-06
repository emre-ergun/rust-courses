fn main() {
    println!("Hello, world!");

    let bunnies = 2;
    let (bunny, carrot) = (1, 5);

    const TEST_CONST: i32 = 5;

    println!("bunnies:{}, bunny:{}, carrot:{}", bunnies, bunny, carrot);
    println!("Test Constant:{}", TEST_CONST);
}
