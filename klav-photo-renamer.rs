extern crate image_meta;

use image_meta;

fn main() {
  let meta = image_meta::load_from_file("test-files/paw.png").unwrap();
  println!("dims: {}x{}", meta.dimensions.width, meta.dimensions.width);
  println!("animation: {:?}", meta.is_animation());
  println!("format: {:?}", meta.format);
}