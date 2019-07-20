#![recursion_limit="256"]

mod footer;
mod content;
mod header;
mod blurb;

mod cratesio;

use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

use footer::Footer;
use content::Main;


pub struct Crates {}

pub enum Msg {
}

impl Component for Crates {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Crates {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }
}

impl Renderable<Crates> for Crates {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <Main />
                <Footer />
            </div>
        }
    }
}
