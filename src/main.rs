mod config;
mod tasks;
mod ui;

use clap::Parser;
use cliclack::{self};
use colored::Colorize;
use serenity::{
    all::{
        ClientBuilder, Context, EditGuild, EventHandler, GatewayIntents, GuildId, HttpBuilder,
        OnlineStatus, Ready,
    },
    async_trait,
};

use config::proxy::PROXY_MANAGER;
use ui::{banner, SPINNER_CLIENT};

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

    SPINNER_CLIENT.start("Starting client");

    PROXY_MANAGER.shuffle_proxies().await;
    let proxies = PROXY_MANAGER.get_all_proxies().await;

    let mut http = HttpBuilder::new(&token).ratelimiter_disabled(true);

    if !proxies.is_empty() {
        http = http.proxy(PROXY_MANAGER.get_next_proxy().await.unwrap());
    }

    let mut client = ClientBuilder::new_with_http(http.build(), GatewayIntents::all())
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
        ctx.set_presence(None, OnlineStatus::Offline);

        SPINNER_CLIENT.stop(format!(
            "{} {}",
            "Logged in as".green(),
            ctx.cache.current_user().name
        ));

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

            cliclack::log::success(format!(
                "{} {}",
                "Logged in as".green(),
                ctx.cache.current_user().name
            ))
            .unwrap();

            let menu = cliclack::select("Menu")
                .items(&[
                    ("tasks", "Tasks", ""),
                    ("config", "Config", ""),
                    ("exit", "Exit", ""),
                ])
                .interact()
                .unwrap();

            match menu {
                "config" => {
                    let config_option = cliclack::select("Config")
                        .items(&[("proxies", "Proxies", ""), ("tokens", "Tokens", "")])
                        .interact()
                        .unwrap();

                    match config_option {
                        "proxies" => {
                            let proxy_action = cliclack::select("Proxy Management")
                                .items(&[("view", "View proxies", ""), ("add", "Add proxy", "")])
                                .interact()
                                .unwrap();

                            fn add_proxy(
                            ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>
                            {
                                Box::pin(async {
                                    let new_proxy = cliclack::input("Enter new proxy")
                                        .interact::<String>()
                                        .unwrap();

                                    PROXY_MANAGER.add_proxy(new_proxy).await.unwrap();

                                    cliclack::log::success("Proxy added successfully!").unwrap();
                                    let back =
                                        cliclack::confirm("Back to menu?").interact().unwrap();

                                    if !back {
                                        let other_proxy =
                                            cliclack::confirm("Do you want to add another proxy?")
                                                .interact()
                                                .unwrap();
                                        if other_proxy {
                                            add_proxy().await;
                                        }
                                    }
                                })
                            }

                            match proxy_action {
                                "view" => {
                                    let proxies = PROXY_MANAGER.get_all_proxies().await;

                                    for proxy in proxies {
                                        cliclack::log::info(proxy.bold()).unwrap();
                                    }

                                    cliclack::confirm("Back to menu?").interact().unwrap();
                                }
                                "add" => add_proxy().await,
                                _ => {}
                            }
                        }
                        "tokens" => {
                            // TODO: tokens
                        }
                        _ => {}
                    }
                }
                "tasks" => {
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
                "exit" => {
                    cliclack::outro(format!("Thank you for using! {}", "Exiting...".red()))
                        .unwrap();
                    std::process::exit(0)
                }
                _ => {}
            }
        }
    }
}
