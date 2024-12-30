use crate::{config::proxy::ProxyManager, tasks};
use cliclack::{
    self, clear_screen, confirm, input,
    log::{info, success},
    outro, select,
};
use colored::Colorize;
use serenity::all::{Context, EditGuild, GuildId};
use tokio::task::spawn_blocking;

use super::banner;

pub struct Menu {
    proxy_manager: ProxyManager,
}

impl Menu {
    pub fn new(proxy_manager: ProxyManager) -> Self {
        Self { proxy_manager }
    }

    pub async fn show_main_menu(
        &self,
        ctx: &Context,
        guild_id: GuildId,
    ) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            spawn_blocking(|| clear_screen()).await??;
            banner();

            let menu_items = [
                ("tasks", "Tasks", ""),
                ("config", "Config", ""),
                ("exit", "Exit", ""),
            ];

            let selected =
                spawn_blocking(move || select("Menu").items(&menu_items).interact()).await??;

            match selected {
                "tasks" => self.show_tasks_menu(ctx, guild_id).await?,
                "config" => self.show_config_menu().await?,
                "exit" => {
                    spawn_blocking(|| {
                        outro(format!("Thank you for using! {}", "Exiting...".red()))
                    })
                    .await??;
                    std::process::exit(0);
                }
                _ => {}
            }
        }
    }

    async fn show_config_menu(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_items = [("proxies", "Proxies", ""), ("tokens", "Tokens", "")];

        let selected =
            spawn_blocking(move || select("Config").items(&config_items).interact()).await??;

        match selected {
            "proxies" => self.show_proxy_menu().await?,
            "tokens" => self.show_token_menu().await?,
            _ => {}
        }

        Ok(())
    }

    async fn show_proxy_menu(&self) -> Result<(), Box<dyn std::error::Error>> {
        let proxy_action = spawn_blocking(|| {
            select("Proxy Management")
                .items(&[("view", "View proxies", ""), ("add", "Add proxy", "")])
                .interact()
        })
        .await??;

        match proxy_action {
            "view" => {
                let proxies = self.proxy_manager.get_all_proxies().await;

                for proxy in proxies {
                    spawn_blocking(move || info(proxy.bold())).await??;
                }

                spawn_blocking(|| confirm("Back to menu?").interact()).await??;
            }
            "add" => self.add_proxy().await?,
            _ => {}
        }
        Ok(())
    }

    async fn add_proxy(&self) -> Result<(), Box<dyn std::error::Error>> {
        let proxy_manager = self.proxy_manager.clone();

        let new_proxy = spawn_blocking(|| input("Enter new proxy").interact::<String>()).await??;

        proxy_manager.add_proxy(new_proxy).await?;

        spawn_blocking(|| success("Proxy added successfully!")).await??;
        let back = spawn_blocking(|| confirm("Back to menu?").interact()).await??;

        if !back {
            let add_another =
                spawn_blocking(|| confirm("Do you want to add another proxy?").interact())
                    .await??;

            if add_another {
                let another_proxy =
                    spawn_blocking(|| input("Enter new proxy").interact::<String>()).await??;

                proxy_manager.add_proxy(another_proxy).await?;
                spawn_blocking(|| success("Proxy added successfully!")).await??;
            }
        }

        Ok(())
    }

    async fn show_token_menu(&self) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: tokens
        Ok(())
    }

    async fn show_tasks_menu(
        &self,
        ctx: &Context,
        guild_id: GuildId,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let tasks_items = [
            ("create_ch", "Create channels", ""),
            ("delete_ch", "Delete all channels", ""),
            ("create_rl", "Create roles", ""),
            ("delete_rl", "Delete all roles", ""),
            ("ban_all", "Ban all", ""),
            ("kick_all", "Kick all", ""),
            ("change_name", "Change guild name", ""),
            ("spam_msg", "Spam message", ""),
        ];

        let task = spawn_blocking(move || select("Select a task").items(&tasks_items).interact())
            .await??;

        match task {
            "create_ch" => {
                let name =
                    spawn_blocking(|| input("Enter channel name").interact::<String>()).await??;
                tasks::create_channels(&name, &ctx, guild_id).await;
            }
            "delete_ch" => {
                tasks::delete_channels(&ctx, guild_id).await;
            }
            "create_rl" => {
                let name =
                    spawn_blocking(|| input("Enter role name").interact::<String>()).await??;
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
                let name =
                    spawn_blocking(|| input("Enter guild name").interact::<String>()).await??;
                let guild_builder = EditGuild::new().name(name);
                guild_id.edit(&ctx.http, guild_builder).await?;
            }
            "spam_msg" => {
                let message =
                    spawn_blocking(|| input("Enter spam message").multiline().interact::<String>())
                        .await??;
                tasks::spam_message(&message, &ctx, guild_id).await;
            }
            _ => {}
        }

        Ok(())
    }
}
