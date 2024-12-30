use clap::Parser;
use cliclack;
use serenity::{
    all::{ClientBuilder, Context, EventHandler, GatewayIntents, OnlineStatus, Ready},
    async_trait,
};

#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    token: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut token = args.token.unwrap_or("".to_string());

    cliclack::clear_screen()?;

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
        println!("Bot is ready!");

        let guilds = ctx.cache.guilds();
        let mut items: Vec<(String, String, String)> = Vec::new();

        for guild in guilds {
            let info = ctx.http.get_guild(guild).await.unwrap();
            items.push((info.id.to_string(), info.name.clone(), String::new()));
        }

        let selectedGuild = cliclack::select("Select a guild")
            .items(&items)
            .interact()
            .unwrap();

        loop {
            cliclack::clear_screen().unwrap();

            let task = cliclack::select("Select a task")
                .items(&TASKS)
                .interact()
                .unwrap();

            match task {
                "create_ch" => {
                    let name = cliclack::input("Enter channel name")
                        .interact::<String>()
                        .unwrap();
                }
                "delete_ch" => {}
                "create_rl" => {
                    let name = cliclack::input("Enter role name")
                        .interact::<String>()
                        .unwrap();
                }
                "delete_rl" => {}
                "ban_all" => {}
                "kick_all" => {}
                "change_name" => {
                    let name = cliclack::input("Enter guild name")
                        .interact::<String>()
                        .unwrap();
                }
                "spam_msg" => {
                    let name = cliclack::input("Enter spam message")
                        .multiline()
                        .interact::<String>()
                        .unwrap();
                }
                _ => {}
            }
        }
    }
}
