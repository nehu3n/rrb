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

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        println!("Bot is ready!");

        let guilds = ctx.cache.guilds();
        let mut items: Vec<(String, String, String)> = Vec::new();

        for guild in guilds {
            let info = ctx.http.get_guild(guild).await.unwrap();
            items.push((
                info.id.to_string(),
                info.name.clone(),
                String::new(),
            ));
        }

        cliclack::select("Select a guild")
            .items(&items)
            .interact()
            .unwrap();
    }
}
