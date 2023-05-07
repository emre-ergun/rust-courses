//how to write comments in Rust

/*
    Comments

    Any program requires comments, and Rust supports a few different varieties:
    
    Regular comments which are ignored by the compiler:

    "// : double line comments which go to the end of a line"
    "/* */ : Block comments go to the closing delimeter"
    "/// : doc comments are parsed into HTML library documentation"
    "//! : generate library documents for a following itemn (enclosing item //!)"
 */

fn main(){
// hello

/*
    hello there
    this is a block comment
 */

    println!("Hello 🌎!");
}