mod app;
mod pages;
mod store;
mod router;
mod components;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
