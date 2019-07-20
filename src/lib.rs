#![recursion_limit="128"]

use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};
use yew::services::ConsoleService;

mod footer;
use footer::Footer;


pub struct Crates {}

pub enum Msg {
}

impl Component for Crates {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Crates {}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }
}

impl Renderable<Crates> for Crates {
    fn view(&self) -> Html<Self> {
        html! {
            <Footer />
        }
    }
}
