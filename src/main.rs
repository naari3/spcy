use dotenv::dotenv;
use std::env;

use log::{debug, error, info};

mod mp3;
mod spc;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        for attachment in &msg.attachments {
            if !attachment.filename.contains(".spc") {
                continue;
            }
            info!("Get spc file: {}", attachment.filename);
            let mut content = match attachment.download().await {
                Ok(content) => content,
                Err(why) => {
                    error!("Error downloading attachment: {:?}", why);
                    let _ = msg
                        .channel_id
                        .say(&ctx, "Error downloading attachment")
                        .await;
                    continue;
                }
            };
            let filename: &str = &format!("{}.mp3", &attachment.filename.replace(".spc", ""));
            debug!("Start spc converting...");
            let mut samples = match spc::spc_to_samples(&mut content) {
                Ok(data) => data,
                Err(why) => {
                    error!("Error converting spc to samples: {:?}", why);
                    // maybe just not spc
                    continue;
                }
            };
            debug!("Finished spc converting");
            debug!("Sample length: {}", samples.len());
            debug!("Start mp3 encoding...");
            let mp3data = match mp3::samples_to_mp3(&mut samples) {
                Ok(data) => data,
                Err(why) => {
                    error!("Error converting samples to mp3: {:?}", why);
                    // maybe just not spc
                    continue;
                }
            };
            debug!("Finished mp3 encoding");
            debug!("mp3data: {}", &mp3data.len());
            let files = vec![(&mp3data[..], filename)];
            match msg.channel_id.send_files(&ctx, files, |m| m).await {
                Ok(_) => {}
                Err(why) => {
                    error!("Error sending mp3 to Discord: {}", &why);
                    let discord_msg = format!("Error: {}", &why);
                    match msg.reply(&ctx, discord_msg).await {
                        Ok(_) => {}
                        Err(why) => {
                            error!("Error sending mp3 error message: {}", why)
                        }
                    };
                    continue;
                }
            };
            info!("Finish: {}", attachment.filename);
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    env_logger::init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
