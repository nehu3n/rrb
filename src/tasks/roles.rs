use serenity::all::EditRole;
use serenity::model::prelude::*;
use serenity::prelude::*;
use tokio::task;

pub async fn create_roles(name: &str, ctx: &Context, guild_id: GuildId) {
    let futures: Vec<_> = (0..200)
        .map(|i| {
            let http = ctx.http.clone();
            let guild_id = guild_id.clone();

            let role_builder = EditRole::new().name(name);

            task::spawn(async move {
                if let Err(err) = guild_id.create_role(&http, role_builder).await {
                    eprintln!("Error creating role {}: {:?}", i, err);
                }
            })
        })
        .collect();

    for future in futures {
        let _ = future.await;
    }
}

pub async fn delete_roles(ctx: &Context, guild_id: GuildId) {
    let guild = guild_id.to_partial_guild(&ctx.http).await.unwrap();

    let roles = guild.roles.clone();
    let futures: Vec<_> = roles
        .values()
        .cloned()
        .map(|mut role| {
            let http = ctx.http.clone();
            task::spawn(async move {
                if let Err(err) = role.delete(&http).await {
                    eprintln!("Error deleting role {}: {:?}", role.id, err);
                }
            })
        })
        .collect();

    for future in futures {
        let _ = future.await;
    }
}