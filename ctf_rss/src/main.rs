use tokio;

mod feed;

#[tokio::main]
async fn main() {
    let feed_map = feed::FeedsMap::new();

    let mut rss_actions: Vec<fn(&rss::Item)> = Vec::new();
    rss_actions.push(feed::print_rss_item);

    let mut rss: Vec<&str> = Vec::new();
    rss.push("ctftime_upcoming");

    feed_map.handle_feeds(rss, rss_actions).await;
}
