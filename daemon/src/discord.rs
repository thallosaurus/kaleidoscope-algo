use std::env;

use serenity::all::ChannelId;
use serenity::all::Http;
use serenity::all::Message;
use serenity::async_trait;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        println!("{msg:?}");
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
            }
        }
    }
}

pub struct DiscordBot {
    //http: Http,
    token: String,
    task: tokio::task::JoinHandle<()>,
}

impl DiscordBot {
    pub fn new(token: String) -> Self {
        let http = Http::new(&token);

        let intents = GatewayIntents::GUILD_MESSAGES
            | GatewayIntents::DIRECT_MESSAGES
            | GatewayIntents::MESSAGE_CONTENT;

            let task_token = token.clone();
        DiscordBot {
            //http,
            token,
            task: tokio::spawn(async move {
                //run task
                let mut client = Client::builder(&task_token, intents)
                    .event_handler(Handler)
                    .await
                    .expect("Error while creating client");

                if let Err(why) = client.start().await {
                    println!("Client error: {why:?}");
                }
            }),
        }
    }

    pub async fn send_message(&self, channel_id: u64, message: String) {
            //let channel_id = ChannelId::new(1051294190455226458);
        let http = Http::new(&self.token);
        let channel = ChannelId::new(channel_id);
        channel.say(http, message).await.unwrap();
    }
}

/*pub async fn discord_bot() -> DiscordBot {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let http = Http::new(&token);

    let channel_id = ChannelId::new(1051294190455226458);
    channel_id.say(http, "the fuck is up").await.unwrap();
}*/
