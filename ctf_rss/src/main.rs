use tokio;
use structopt::StructOpt;

mod feed;


#[derive(Debug, StructOpt)]
struct Cli {
    rss: String,
}

#[tokio::main]
async fn main() {
    let feed_map = feed::FeedsMap::new();

    let args = Cli::from_args();

    let mut rss_actions: Vec<fn(&rss::Item)> = Vec::new();
    rss_actions.push(feed::print_rss_item);

    let mut rss: Vec<&str> = Vec::new();
    // rss.push("ctftime_upcoming");
    // rss.push("ctftime_active");
    rss.push(&args.rss);

    feed_map.handle_feeds(rss, rss_actions).await;
}
