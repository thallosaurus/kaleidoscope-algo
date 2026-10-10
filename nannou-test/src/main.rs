use nannou::prelude::*;

struct Model {
    texture: wgpu::Texture
}

/// Starts the Nannou example that displays a texture.
fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

/// Creates the application state and sets up the window and required rendering resources.
fn model(app: &App) -> Model {
    app.new_window().size(512, 512).view(view).build().unwrap();
    let assets = app.assets_path().unwrap();
    let img_path = assets.join("images").join("test.png");
    println!("{:?}", img_path);
    let texture = wgpu::Texture::from_path(app, img_path).unwrap();

    Model { texture }
}

/// Updates the application state for a Nannou frame.
fn update(_app: &App, _model: &mut Model, _update: Update) {

}

/// Draws the current application state into the given frame.
fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);

    let win = app.window_rect();
    let r = Rect::from_w_h(512.0, 512.0).top_left_of(win);

    let draw = app.draw();
    draw.texture(&model.texture)
    .xy(r.xy())
    .wh(r.wh());
    draw.to_frame(app, &frame).unwrap();
}
