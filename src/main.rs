use image::GenericImageView;
use image::imageops::FilterType;

fn main() {
    println!("Hello, world!");
    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let img = image::open("assets/ScreenShot-RustGoP.png").unwrap();
    // The dimensions method returns the images width and height.
    println!("Before resize dimensions {:?}", img.dimensions());
    // The color method returns the image's `ColorType`.
    println!("Color Type {:?}", img.color());

    let scaled = img.resize(1410, 2250, FilterType::Lanczos3);
    // The dimensions method returns the images width and height.
    println!("After resize dimensions {:?}", scaled.dimensions());

    // Write the contents of this image to the Writer in desired format.
    scaled.save("assets/ScreenShot-RustGoJ.jpg").unwrap();
}
