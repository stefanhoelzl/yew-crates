use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};
use num_format::{Locale, ToFormattedString};

pub struct Blurb {
    downloads: Option<i32>,
    crates: Option<i32>,
}

pub enum Msg {}

#[derive(PartialEq, Clone)]
pub struct BlurbProperties {
    pub downloads: Option<i32>,
    pub crates: Option<i32>,
}

impl Default for BlurbProperties {
    fn default() -> Self {
        BlurbProperties {
            downloads: None,
            crates: None,
        }
    }
}

impl Component for Blurb {
    type Message = Msg;
    type Properties = BlurbProperties;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Blurb {
            downloads: props.downloads,
            crates: props.crates,
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.downloads = props.downloads;
        self.crates = props.crates;
        true
    }
}

impl Renderable<Blurb> for Blurb {
    fn view(&self) -> Html<Self> {
        html! { 
            <div id="blurb">
                <div id="intro">
                    {" Instantly publish your crates and install them. Use the API to
                    interact and find out more information about available crates. Become
                    a contributor and enhance the site with your work. "}
                </div>

                <div id="stats">
                    <div class="downloads">
                        <img src="download.svg" />
                        <span class="num">{self.downloads.unwrap_or(0).to_formatted_string(&Locale::en)}</span>
                        <span class="desc small">{"Downloads"}</span>
                    </div>
                    <div class="crates">
                        <img src="crate.svg" />
                        <span class="num">{self.crates.unwrap_or(0).to_formatted_string(&Locale::en)}</span>
                        <span class="desc small">{"Crates in stock"}</span>
                    </div>
                </div>
            </div>
        }
    }
}
