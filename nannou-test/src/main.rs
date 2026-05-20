use nannou::prelude::*;

struct Model {
    texture: wgpu::Texture
}

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

fn model(app: &App) -> Model {
    app.new_window().size(512, 512).view(view).build().unwrap();
    let assets = app.assets_path().unwrap();
    let img_path = assets.join("images").join("test.png");
    println!("{:?}", img_path);
    let texture = wgpu::Texture::from_path(app, img_path).unwrap();

    Model { texture }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {

}

fn view(_app: &App, _model: &Model, frame: Frame) {
    frame.clear(PURPLE);
}