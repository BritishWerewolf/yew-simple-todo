mod app;
mod todos;
mod store;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
