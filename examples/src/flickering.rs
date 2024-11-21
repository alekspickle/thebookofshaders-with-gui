use nannou::prelude::*;
// use rand;

fn main() {
    nannou::sketch(view).run()
}

fn view(app: &App, frame: Frame) {
    // Begin drawing
    let draw = app.draw();
    // Clear the background to blue.
    draw.background().color(CORNFLOWERBLUE);

    // Get window as Rect.
    let _win = app.window_rect();
    // Get time as float.
    let t = app.time;

    // Draw a quad that follows the inverse of the ellipse.
    draw.quad()
        .x_y(app.mouse.x * t.cos() * PI, app.mouse.y * t.sin())
        .w_h(500., 500.)
        // .rotate(t)
        .color(CORNFLOWERBLUE)
        .stroke_weight(5.)
        .stroke_color(MEDIUMSPRINGGREEN);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
