struct House {
    door: String,
    garden: String,
    property_value: u32,
}

fn main() {

    //struct example
    let home1 = House{
        door: String::from("Blue"),
        garden: String::from("Beautiful"),
        property_value: 1000000
    };
    
    println!("door is: {}, garden is: {}, property value is: {}",
    home1.door, home1.garden, home1.property_value);

    //exercise struct
    struct Triangle {
        base: u32,
        height: u32,
    }

    impl Triangle {
        fn calculate_area(&self) -> u32
        {
            return self.base * self.height / 2;
        }
    }

    let a_triangle = Triangle {
        base: 10,
        height: 30,
    };

    println!("The are of the triangle(base:{}, height:{}) is {}", 
    a_triangle.base, a_triangle.height, a_triangle.calculate_area());

    //enum example
    #[derive(Debug)] //the derive attribute makes the enum printable
    enum TemperatureGrade {
        Hot,Cold,Medium,
    }

    #[derive(Debug)]
    struct City {
        name: String,
        temperature: TemperatureGrade,
    }

    let c1 = City {
        name: String::from("Alaska"),
        temperature: TemperatureGrade::Cold,
    };
    
    let c2 = City{
        name: String::from("Miami"),
        temperature: TemperatureGrade::Hot,
    };

    println!("{:?}", c1);
    println!("{:?}", c2);

    //matching enum exercise
    #[derive(Debug)]
    enum Shoes {
        Loafers,
        Nike,
        Vans,
    }

    fn check_shoes(val: &Shoes) {
        match val {
            Shoes::Loafers => {
                println!("Great for loafing around");
            },
            Shoes::Nike => {
                println!("Great for running");
            },
            Shoes::Vans => {
                println!("Great for skateboarding");
            }

        }
    }

    check_shoes(&Shoes::Loafers);
    check_shoes(&Shoes::Nike);
    check_shoes(&Shoes::Vans);

    let shoe = Shoes::Nike;
    check_shoes(&shoe);
    println!("{:?}", shoe);
    use inter_in_rust_second::songs::play;
    let song_name = String::from("Hello");
    play(song_name);


    //exercise for modules
    pub mod tracks {
        pub mod rock {
            pub mod indie {
                pub fn select(val: String) {
                    println!("{}", val);
                }
            }
        }
    }

    use tracks::rock::indie::select;

    select("Serenade".to_string());
    select("French Baguette".to_string());
    select("Pineapple Blues".to_string());

    use std::collections::HashMap;

    let mut account_info = HashMap::new();
    account_info.insert("Johny", "Overdraft!");
    account_info.insert("Sally", "Good Standing!");
    account_info.insert("Superman", "Insufficient funds!");
    println!("The size of the map is {}", account_info.len());

    //hash map exercise
    let mut bar_drinks = HashMap::new();
    bar_drinks.insert("vodka", true);
    bar_drinks.insert("beer", false);
    bar_drinks.insert("whiskey", true);
    println!("The size of the map is {}", bar_drinks.len());
    
    bar_drinks.remove(&"whiskey");
    println!("The size of the map is {}", bar_drinks.len());

}

