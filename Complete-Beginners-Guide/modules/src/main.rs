mod player;
mod archive;
use std::println;

//use crate::archive::arch::arch_file;
use crate::archive::arch::arch_file as arch; //alias

use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

fn main() {
	let name = "Matrix.mp4";
	player::play_movie(name);
	let audio = "bbc_radio.mp3";
	player::play_audio(audio);
    clean::clean_system();
    clean::files::clean_files();

    player::delete::delete_system();
    player::delete::files::delete_files();

    //arch_file("secret_file.txt");
    arch("secret_file.txt");

    let mut rng = rand::thread_rng();
    let a: i32 = rng.gen();
    let b: i32 = rng.gen_range(0, 11);
    println!("Random number: {}", a);
    println!("Random number(1..10): {}", b);

    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .collect();

    println!("Random string: {}", rand_string);
}


mod clean {
    pub fn clean_system() {
        println!("System cleaning");
    }
    pub mod files {
        pub fn clean_files() {
            println!("Files cleaning");
        }
    }
}
