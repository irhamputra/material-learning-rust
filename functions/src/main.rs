// Silence some warnings so they don't distract from the exercise.
#![allow(unused_variables)]

fn main() {
    let width = 4;
    let height = 7;
    let depth = 10;

    {
        // inner scope!
        // either way remove the curly braces or put println!() in the same scope of the area variables
        let area = area_of(width, height);
        println!("Area is {}", area);
    }

    let result = volume(width, height, depth);
    println!("Volume is {}", result);
}

fn area_of(x: i32, y: i32) -> i32 {
    // cargo clippy will check return keyword
    // in rust return must follow by semicolon in the end
    x * y // without return keyword this means tail expression
}

fn volume(x: i32, y: i32, z: i32) -> i32 {
    x * y * z
}
