pub fn play_movie(name: &str) {
    println!("The playing movie: {}", name);
}

pub fn play_audio(name: &str) {
    println!("The playing audio: {}", name);
}


pub mod delete {
    pub fn delete_system() {
        println!("System deleting");
    }
    pub mod files {
        pub fn delete_files() {
            println!("Files deleting");
        }
    }
}