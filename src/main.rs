use std::env;

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
        for attachment in msg.attachments {
            let mut content = match attachment.download().await {
                Ok(content) => content,
                Err(why) => {
                    println!("Error downloading attachment: {:?}", why);
                    let _ = msg
                        .channel_id
                        .say(&ctx, "Error downloading attachment")
                        .await;
                    return;
                }
            };
            let filename: &str = &format!("{}.mp3", attachment.filename);
            let mut samples = match spc::spc_to_samples(&mut content) {
                Ok(data) => data,
                Err(why) => {
                    println!("Error converting spc to samples: {:?}", why);
                    // let _ = msg.channel_id.say(&ctx, "Error converting spc to samples").await;
                    // maybe just not spc
                    return;
                }
            };
            println!("sample length: {}", samples.len());
            let mp3data = match mp3::samples_to_mp3(&mut samples) {
                Ok(data) => data,
                Err(why) => {
                    println!("Error converting samples to mp3: {:?}", why);
                    // let _ = msg.channel_id.say(&ctx, "Error converting samples to mp3").await;
                    // maybe just not spc
                    return;
                }
            };
            println!("mp3data: {}", &mp3data.len());
            let files = vec![(&mp3data[..], filename)];
            let _ = msg.channel_id.send_files(&ctx, files, |m| m).await;
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
