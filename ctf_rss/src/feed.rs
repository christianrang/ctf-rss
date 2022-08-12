use std::collections::HashMap;
use std::error::Error;

use reqwest;
use rss::Channel;

const CTFTIME_UPCOMING_RSS: &str = "https://ctftime.org/event/list/upcoming/rss/";

pub fn print_rss_item(item: &rss::Item) {
    println!(
        "Title:        {}",
        item.title().expect("couldn't find string")
    );
    println!(
        "Link:         {}",
        item.link().expect("couldn't find string")
    );
    println!(
        "Description:\n  {}",
        item.description().expect("couldn't find string")
    );
    println!();
}

pub async fn read_feed(url: &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(url).await?.bytes().await?;

    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

pub struct FeedsMap {
    feeds: HashMap<String, String>,
}

impl FeedsMap {
    pub fn new<'a>() -> FeedsMap {
        FeedsMap {
            feeds: HashMap::from([(
                String::from("ctftime_upcoming"),
                String::from(CTFTIME_UPCOMING_RSS),
            )]),
        }
    }

    pub fn get(&self, value: &str) -> Option<&String> {
        self.feeds.get(value)
    }

    pub async fn handle_feeds(&self, rss: Vec<&str>, actions: Vec<fn(&rss::Item)>) {
        for r in rss {
            match self.get(r) {
                Some(value) => {
                    let channel = read_feed(&value)
                        .await
                        .expect("failed to create channel in main");

                    for i in channel.items() {
                        for action in &actions {
                            action(&i);
                        }
                    }
                }
                _ => println!("Couldn't find value"),
            };
        };
    }
}
