use macroquad::prelude::*;

#[macroquad::main("Foo")]
async fn main() {
    info!("before load_file on foo.txt");
    let result = load_file("foo.txt").await;
    info!("got {:?}", result);

    info!("before load_file on nonexistant.txt");
    let result = load_file("nonexistant.txt").await;
    info!("got {:?}", result);
}
