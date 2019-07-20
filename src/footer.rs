use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Footer {}

pub enum Msg {}

impl Component for Footer {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Footer {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender { false }
}

impl Renderable<Footer> for Footer {
    fn view(&self) -> Html<Self> {
        html! {
            <footer class="after-main-links">
                <a href="https://doc.rust-lang.org/cargo/getting-started/installation.html">{"Install"}</a>
                <span class="sep">{"|"}</span>
                <a href="https://doc.rust-lang.org/cargo/">{"Getting Started"}</a>
                <span class="sep">{"|"}</span>
                <a href="https://doc.rust-lang.org/cargo/guide/">{"Guide"}</a>
                <span class="sep">{"|"}</span>
                <a href="mailto:help@crates.io">{"Send us an email"}</a>
                <span class="sep">{"|"}</span>
                <a href="https://crates.io/policies">{"Policies"}</a>
            </footer>
        }
    }
}
