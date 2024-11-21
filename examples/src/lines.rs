use nannou::prelude::*;
use rand;

fn main() {
    nannou::sketch(view).run()
}

fn view(app: &App, frame: Frame) {
    // Begin drawing
    let draw = app.draw();
    // Clear the background to blue.
    draw.background().color(CORNFLOWERBLUE);

    // Get window as Rect.
    let win = app.window_rect();
    // Get time as float.
    let t = app.time;

    // Step
    let step: f32 = 80.;

    render(&draw, app.mouse.x, app.mouse.y, step, step);
        
    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}

fn render(draw: &Draw, x: f32, y: f32, w: f32, h: f32) {
    // left to right bool
    let left_to_right = rand::random::<f32>();

    if left_to_right >= 0.5 {
        draw.line()
            .weight(1.)
            .points(Point2::new(x, y), Point2::new(x + w, y + h));
    } else {
        draw.line()
            .weight(1.)
            .points(Point2::new(x, y), Point2::new(y + w, x + h));
    }
}
