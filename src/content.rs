use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

use crate::header::Header;


pub struct Main {}

pub enum Msg {}

impl Component for Main {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Main {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }
}

impl Renderable<Main> for Main {
    fn view(&self) -> Html<Self> {
        html! { 
            <main id="main" class="inner-content">
                <p id="flash"/>
                <Header />
            </main>
        }
    }
}
