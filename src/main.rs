mod config;
mod tasks;
mod ui;

use clap::Parser;
use cliclack::{clear_screen, input, select};
use colored::Colorize;
use serenity::{
    all::{
        ClientBuilder, Context, EventHandler, GatewayIntents, GuildId, Http, HttpBuilder,
        OnlineStatus, Ready,
    },
    async_trait,
};

use ui::{banner, Menu, SPINNER_CLIENT};
use config::proxy::ProxyManager;

#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    token: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    clear_screen()?;
    banner();

    let token = get_token(args.token).await.unwrap();
    let proxy_manager = ProxyManager::new().unwrap();

    SPINNER_CLIENT.start("Starting client");

    let http = configure_http(&token, &proxy_manager).await.unwrap();
    let handler = Handler {
        menu: Menu::new(proxy_manager),
    };

    let mut client = ClientBuilder::new_with_http(http, GatewayIntents::all())
        .status(OnlineStatus::Offline)
        .event_handler(handler)
        .await?;

    if let Err(e) = client.start().await {
        println!("An error occurred while running the client: {:?}", e);
    }

    Ok(())
}

async fn get_token(token_arg: Option<String>) -> Result<String, ()> {
    if let Some(token) = token_arg {
        Ok(token)
    } else {
        let token = input("Enter bot token")
            .required(true)
            .validate_on_enter(|v: &String| {
                if v.is_empty() {
                    Err("Token cannot be empty")
                } else {
                    Ok(())
                }
            })
            .interact::<String>()
            .unwrap();

        Ok(token)
    }
}

async fn configure_http(token: &str, proxy_manager: &ProxyManager) -> Result<Http, ()> {
    let mut http = HttpBuilder::new(token).ratelimiter_disabled(true);

    if !proxy_manager.get_all_proxies().await.is_empty() {
        proxy_manager.shuffle_proxies().await;
        let proxy = proxy_manager.get_next_proxy().await.unwrap();
        http = http.proxy(&proxy);
    }

    Ok(http.build())
}

struct Handler {
    menu: Menu,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        ctx.set_presence(None, OnlineStatus::Offline);

        SPINNER_CLIENT.stop(format!(
            "{} {}",
            "Logged in as".green(),
            ctx.cache.current_user().name
        ));

        let guild_id = self.select_guild(&ctx).await;
        self.menu.show_main_menu(&ctx, guild_id).await.unwrap();
    }
}

impl Handler {
    async fn select_guild(&self, ctx: &Context) -> GuildId {
        let guilds = ctx.cache.guilds();
        let mut items: Vec<(String, String, String)> = Vec::new();

        for guild in guilds {
            let info = ctx.http.get_guild(guild).await.unwrap();
            items.push((info.id.to_string(), info.name.clone(), String::new()));
        }

        let selected_guild = select("Select a guild").items(&items).interact().unwrap();

        GuildId::new(selected_guild.parse::<u64>().unwrap())
    }
}
