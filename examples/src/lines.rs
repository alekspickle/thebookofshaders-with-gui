use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run()
}

fn view(app: &App, frame: Frame) {
    // Begin drawing
    let draw = app.draw();

    // Clear the background to blue.
    draw.background().color(CORNFLOWERBLUE);

    // Draw a purple triangle in the top left half of the window.
    let _win = app.window_rect();

    // Draw an ellipse to follow the mouse.
    let _t = app.time;

    // Main render task
    render(&draw, app.mouse.x, app.mouse.y);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}

fn render(draw: &Draw, x: f32, y: f32) {
    // Draw a quad that follows the inverse of the ellipse.
    draw.quad()
        .x_y(x,y)
        .w_h(500., 500.)
        // .rotate(t)
        .color(CORNFLOWERBLUE)
        .stroke_weight(10.)
        .stroke_color(MEDIUMSPRINGGREEN);

}