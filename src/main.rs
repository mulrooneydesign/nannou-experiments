use nannou::prelude::*;

struct Model {
    _window: window::Id,
}

fn main() {
    nannou::app(model).event(event).run();
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(1024, 1024)
        .view(view)
        .build()
        .unwrap();

    Model { _window }
}

fn event(_app: &App, _model: &mut Model, _event: Event) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    let time = app.elapsed_frames() as f32 / 60.0;

    // Only erase  background for the first frame
    // Other frame will be drawn upon continually
    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }

    for i in 0..50 {
        let angle = i as f32 * 0.1 * TAU * time;

        let w_h = random::<f32>() * 2.0;

        draw.ellipse()
            .w_h(w_h, w_h)
            .x_y(
                random::<f32>() * 1024.0 * angle.cos(),
                random::<f32>() * 1024.0 * angle.sin() * (time / 2.0),
            )
            .color(WHITE);
    }

    for i in 0..10 {
        let angle = i as f32 * 0.1 * TAU * time;

        let w_h = 5.0 * random::<f32>();

        draw.ellipse()
            .w_h(w_h, w_h)
            .x_y(200.0 * angle.cos(), 200.0 * angle.sin())
            .color(GREEN);
    }

    // draw faint rect each frame.
    draw.rect()
        .w_h(1024.0, 1024.0)
        .color(srgba(0.0, 0.0, 0.0, 0.05));

    for i in 0..1000 {
        let angle = i as f32 * 0.1 * TAU * time;

        draw.ellipse()
            .w_h(1.0, 1.0)
            .x_y(400.0 * angle.cos() * time, 400.0 * -angle.sin() * time)
            .color(GREENYELLOW);
    }

    draw.to_frame(app, &frame).unwrap();
}
