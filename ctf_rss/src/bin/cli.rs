use tokio;
use structopt::StructOpt;

use ctf_rss::feed;

#[derive(Debug, StructOpt)]
struct Cli {
    rss: String,
    #[structopt(short, long)]
    verbose: bool,
    #[structopt(short, long, default_value="")]
    title: String,
}

#[tokio::main]
async fn main() {
    let feed_map = feed::FeedsMap::new();

    let args = Cli::from_args();

    let mut rss_actions: Vec<fn(&rss::Item)> = Vec::new();
    if args.verbose {
        rss_actions.push(feed::print_rss_item);
    } else if !args.title.is_empty() {
        rss_actions.push(feed::print_rss_item);
    } else {
        rss_actions.push(feed::print_rss_title);
    }

    let mut rss: Vec<&str> = Vec::new();
    rss.push(&args.rss);

    if args.title.is_empty() {
        feed_map.handle_feeds(rss, rss_actions).await;
    } else {
        feed_map.handle_feeds_with_filter(rss, rss_actions, args.title).await;
    }
}
