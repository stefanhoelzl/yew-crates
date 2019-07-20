use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Header {}

pub enum Msg {}

impl Component for Header {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Header {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender { false }
}

impl Renderable<Header> for Header {
    fn view(&self) -> Html<Self> {
        html! { 
            <div id="title-header">
                <h1>{"The Rust community's crate registry"}</h1>
            
                <div class="links">
                    <a href="https://doc.rust-lang.org/cargo/getting-started/installation.html" class="yellow-button">
                        <img src="button-download.svg" />
                        {" Install Cargo "}
                    </a>
                    <a href="https://doc.rust-lang.org/cargo/guide/" class="yellow-button">
                        <img src="flag.svg" />
                        {" Getting Started "}
                    </a>
                </div>
            </div>
        }
    }
}
