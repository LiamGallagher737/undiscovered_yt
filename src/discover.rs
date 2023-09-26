use anyhow::Result;
use chrono::{DateTime, Datelike, Local};
use futures::executor::block_on;
use rand::seq::SliceRandom;
use rand::thread_rng;
use yt_api::search::{ItemType, Order, SearchList, SearchResult};
use yt_api::ApiKey;

pub enum Discovery {
    Webcam,
    Pc,
    SmartPhone,
    Misc,
}

impl Discovery {
    pub const VARIANTS: &'static [&'static str] = &["Webcam", "Pc", "Smartphone", "Misc"];

    pub fn from_index(n: usize) -> Self {
        match n {
            0 => Self::Webcam,
            1 => Self::Pc,
            2 => Self::SmartPhone,
            3 => Self::Misc,
            _ => panic!("Invalid discovery index"),
        }
    }

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

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Extra {
    Date,
}

impl Extra {
    pub const VARIANTS: &'static [&'static str] = &["Date"];

    pub fn from_index(n: usize) -> Self {
        match n {
            0 => Self::Date,
            _ => panic!("Invalid extra index"),
        }
    }

    fn apply(&self, query: &mut String) {
        match self {
            Extra::Date => {
                let date: DateTime<Local> = Local::now();
                query.push(' ');
                query.push_str(&format!(
                    "{}{:0>2}{:0>2}",
                    date.year(),
                    date.month(),
                    date.day()
                ));
            }
        }
    }
}

pub fn search(
    discovery: Discovery,
    extras: Vec<Extra>,
    results: usize,
    api_key: String,
) -> Result<Vec<SearchResult>> {
    let mut rng = thread_rng();

    let options = discovery.query_options();
    let option = *options.choose(&mut rng).unwrap();
    let mut query = String::from(option);

    for extra in extras {
        extra.apply(&mut query);
    }

    let future = SearchList::new(ApiKey::new(api_key))
        .q(query)
        .max_results(results as u8)
        .order(Order::Date)
        .item_type(ItemType::Video);
    let result = block_on(future)?;

    Ok(result.items)
}
