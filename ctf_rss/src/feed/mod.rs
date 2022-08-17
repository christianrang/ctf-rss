use std::collections::HashMap;
use std::error::Error;

use reqwest;
use rss::Channel;
use html2text;
use terminal_size::{Width, Height, terminal_size};

const CTFTIME_UPCOMING_RSS: &str = "https://ctftime.org/event/list/upcoming/rss/";
const CTFTIME_ACTIVE_RSS: &str = "https://ctftime.org/event/list/running/rss/";

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
        "Description:\n{}",
        convert_html_to_text(item.description().expect("couldn't find string")),
    );
    println!("");
}

pub fn print_rss_title(item: &rss::Item) {
    println!(
        "Title: {}",
        item.title().expect("couldn't find title")
    );
}

fn convert_html_to_text(html: &str) -> String {
    // BUG: this cannot find the size of terminal when piped into less.
    // The way it is now it will use the None match arm in that case.
    let (Width(term_width), _) = match terminal_size() {
        Some(value) => value,
        None => (Width(200), Height(200)),
    };
    html2text::from_read(html.as_bytes(), term_width.into())
}

pub async fn read_feed(url: &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(url).await?.bytes().await?;

    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

pub struct FeedsMap {
    feeds: HashMap<String, String>,
    channel: rss::Channel,
}

impl FeedsMap {
    pub fn new<'a>() -> FeedsMap {
        FeedsMap {
            feeds: HashMap::from([
                (
                    String::from("ctftime_upcoming"),
                    String::from(CTFTIME_UPCOMING_RSS),
                ),
                (
                    String::from("ctftime_active"),
                    String::from(CTFTIME_ACTIVE_RSS),
                ),
            ]),
            channel: rss::Channel::default(),
        }
    }

    pub fn get(&self, value: &str) -> Option<&String> {
        self.feeds.get(value)
    }

    pub async fn handle_feeds(&mut self, rss: Vec<&str>, actions: Vec<fn(&rss::Item)>) {
        for r in rss {
            match self.get(r) {
                Some(value) => {
                    self.channel = read_feed(&value)
                        .await
                        .expect("failed to create channel in main");

                    for i in self.channel.items() {
                        for action in &actions {
                            action(i);
                        }
                    }
                }
                _ => println!("Couldn't find value"),
            };
        }
    }

    pub async fn handle_feeds_with_filter(&self, rss: Vec<&str>, actions: Vec<fn(&rss::Item)>, filter: String) {
        for r in rss {
            match self.get(r) {
                Some(value) => {
                    let channel = read_feed(&value)
                        .await
                        .expect("failed to create channel in main");

                    for i in channel.items() {
                        for action in &actions {
                            if i.title().expect("couldn't retrieve title").contains(&filter) {
                                action(&i);
                            }
                        }
                    }
                }
                _ => println!("Couldn't find value"),
            };
        }
    }
}
