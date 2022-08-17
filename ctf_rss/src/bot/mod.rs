use serde::{Deserialize, Serialize};
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    bot: BotConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BotConfig {
    discord_token: String,
}

struct Handler;

const HELP_COMMAND: &str = "!help";
const UPCOMING_COMMAND: &str = "!upcoming";
const ACTIVE_COMMAND: &str = "!active";

// TODO: write helpful help message
const HELP_MESSAGE: &str = "
Hi human
";
const UNRECOGNIZED_COMMAND: &str = "
Sorry I don't recognize that command 
";

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == HELP_COMMAND {
            if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
                println!("Error sending message: {:?}", why);
            }
        // } else if msg.content == UPCOMING_COMMAND {
        //     if let Err(why) = msg.channel_id.say(&ctx.http, upcoming_message).await {
        //         println!("Error sending message: {:?}", why);
        //     }
        } else {
            if let Err(why) = msg.channel_id.say(&ctx.http, UNRECOGNIZED_COMMAND).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
