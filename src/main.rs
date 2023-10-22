mod app;
mod dark_mode;
mod todos;
mod store;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
