use bevy::prelude::*;

fn hello_world() {
    println!("hello world");
}

fn main() {
    App::build()
        .add_system(hello_world.system())
        .run();
}
