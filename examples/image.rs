use plotlars::{Axis, Image, Plot};

fn main() {
    let axis = Axis::new().show_axis(false);

    Image::builder()
        .path("data/image.png")
        .x_axis(&axis)
        .y_axis(&axis)
        .plot_title("Image Plot")
        .build()
        .plot();
}
