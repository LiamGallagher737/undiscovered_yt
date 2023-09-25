use crate::app::App;
use anyhow::{anyhow, Result};
use futures::executor::block_on;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::thread;
use yt_api::search::{Order, SearchList, SearchResult};
use yt_api::ApiKey;

pub enum Discovery {
    Webcam,
    Pc,
    SmartPhone,
    Misc,
}

impl Discovery {
    pub const OPTIONS: &'static [&'static str] = &["webcam", "pc", "smartphone", "misc"];
    pub const _OPTION_COUNT: usize = Self::OPTIONS.len();

    pub fn query_options(self) -> &'static [&'static str] {
        match self {
            Discovery::Webcam => &["WIN", "VID"],
            Discovery::Pc => &[
                ".MP4", ".3GP", ".MOV", ".AVI", ".WMV", ".MKV", ".MPEG", ".FLV",
            ],
            Discovery::SmartPhone => &["IMG", "MVI", "WhatsApp Video"],
            Discovery::Misc => &["FullSizeRender", "My Movie"],
        }
    }
}

pub fn start_discovering(app: &mut App, discovery: Discovery) {
    let mut rng = thread_rng();

    let options = discovery.query_options();
    let option = *options.choose(&mut rng).unwrap();

    let api_key = app.config.api_key.clone();
    let results = app.results.clone();
    let thread = thread::spawn(move || {
        let future = handle_search(option.to_string(), api_key);
        let search_results = block_on(future);
        match search_results {
            Ok(mut data) => {
                results.lock().unwrap().append(&mut data);
            }
            Err(_) => {
                panic!("err getting results")
            }
        }
    });

    if let Some(_handle) = &app.discover_thread {
        // TODO: Stop thread from adding more results
    }

    app.discover_thread = Some(thread);
}

pub async fn handle_search(query: String, api_key: String) -> Result<Vec<SearchResult>> {
    let result = SearchList::new(ApiKey::new(api_key))
        .q(query)
        .max_results(5)
        .order(Order::Date)
        .await;
    match result {
        Ok(response) => Ok(response.items),
        Err(e) => Err(anyhow!("Failed to get results: {e:?}")),
    }
}
