use failure::Error;
use yew::{html, Callback, Component, ComponentLink, Html, Renderable, ShouldRender};
use yew::services::ConsoleService;
use yew::services::fetch::FetchTask;

use crate::header::Header;
use crate::blurb::Blurb;
use crate::cratesio::{CratesIoService, Summary};


pub struct Main {
    console: ConsoleService,
    cratesio: CratesIoService,
    callback: Callback<Result<Summary, Error>>,
    task: Option<FetchTask>,
    summary: Option<Summary>,
}

pub enum Msg {
    SummaryReady(Result<Summary, Error>)
}

impl Component for Main {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let mut main = Main {
            console: ConsoleService::new(),
            cratesio: CratesIoService::new(),
            callback: link.send_back(Msg::SummaryReady),
            task: None,
            summary: None
        };
        main.task = Some(main.cratesio.summary(main.callback.clone()));
        main
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SummaryReady(Ok(summary)) => {
                self.summary = Some(summary);
                self.console.log("Summary successfully fetched");
            }
            Msg::SummaryReady(Err(error)) => {
                self.console.log(format!("{}", error).as_str());
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender { false }
}

impl Renderable<Main> for Main {
    fn view(&self) -> Html<Self> {
        let mut downloads = None;
        let mut crates = None;

        match &self.summary {
            Some(summary) => {
                downloads = Some(summary.num_downloads);
                crates = Some(summary.num_crates);
            },
            None=> {}
        }


        html! { 
            <main id="main" class="inner-content">
                <p id="flash"/>
                <Header />
                <Blurb crates={downloads} downloads={crates}/>
            </main>
        }
    }
}
