//! aphyn.org — the website of the Aphyn game engine, built with Yew.

mod app;
mod components;
mod i18n;
mod pages;
mod releases;
mod route;
mod theme;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
