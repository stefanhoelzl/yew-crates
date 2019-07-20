use failure::{format_err, Error};
use serde_derive::Deserialize;
use yew::callback::Callback;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

#[derive(Deserialize, Debug)]
pub struct Summary {
    pub num_downloads: i32,
    pub num_crates: i32,
    new_crates: Vec<CrateInfo>,
    most_downloaded: Vec<CrateInfo>,
    most_recently_downloaded: Vec<CrateInfo>,
    just_updated: Vec<CrateInfo>,
    popular_keywords: Vec<KeywordInfo>,
    popular_categories: Vec<CategoryInfo>,
}

#[derive(Deserialize, Debug)]
pub struct KeywordInfo {
    id: String,
    keyword: String,
    created_at: String,
    crates_cnt: i32,
}

#[derive(Deserialize, Debug)]
pub struct CategoryInfo {
    id: String,
    category: String,
    slug: String,
    description: String,
    created_at: String,
    crates_cnt: i32,
}

#[derive(Deserialize, Debug)]
pub struct CrateInfo {
    id: String,
    name: String,
    updated_at: String,
    versions: Option<Vec<String>>,
    keywords: Option<Vec<String>>,
    categories: Option<Vec<String>>,
    badges: Option<Vec<String>>,
    created_at: String,
    downloads: i32,
    recent_downloads: Option<i32>,
    max_version: String,
    description: String,
    homepage: Option<String>,
    documentation: Option<String>,
    repository: Option<String>,
    links: Links,
    exact_match: bool,
}

#[derive(Deserialize, Debug)]
pub struct Links {
    version_downloads: String,
    versions: String,
    owners: String,
    owner_team: String,
    owner_user: String,
    reverse_dependencies: String,

}

#[derive(Default)]
pub struct CratesIoService {
    web: FetchService,
}

impl CratesIoService {
    pub fn new() -> Self {
        Self {
            web: FetchService::new(),
        }
    }

    pub fn summary(&mut self, callback: Callback<Result<Summary, Error>>) -> FetchTask {
        let url = "/summary";
        let handler = move |response: Response<Json<Result<Summary, Error>>>| {
            let (meta, Json(data)) = response.into_parts();
            if meta.status.is_success() {
                callback.emit(data)
            } else {
                // format_err! is a macro in crate `failure`
                callback.emit(Err(format_err!(
                    "{}: error getting summary",
                    meta.status
                )))
            }
        };
        let request = Request::get(url).body(Nothing).unwrap();
        self.web.fetch(request, handler.into())
    }
}