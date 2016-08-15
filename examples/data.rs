extern crate adam;

use adam::data::Rectangle;

fn main() {
    // Simple initialization
    let rect = Rectangle {
        x: 0.0,
        y: 0.0,
        w: 200.0,
        h: 100.0,
    };

    // Some utility methods
    let center = rect.center();
    let centered = rect.centered_at((500.0, 500.0));
    let sdl_rect = centered.to_sdl();

    println!("Simple rectangle: {:?}", rect);
    println!("Center of that rectangle: {:?}", center);
    println!("Centered version of that rectangle: {:?}", centered);
    println!("SDL instance of that rectangle: {:?}", sdl_rect);

    // Shortcuts
    let square = Rectangle::with_size(100.0, 100.0);
    println!("Square rectangle: {:?}", square);
}
