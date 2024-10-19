use crate::auth::authenticate_user;
use crate::user_info::{fetch_user_info, send_user_info};
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Activity;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::user::OnlineStatus;
use serenity::prelude::*;

struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            if command.data.name == "authenticate" {
                let user_id = command.user.id;
                let content = if authenticate_user(&ctx, user_id).await {
                    if let Some(user_info) = fetch_user_info(&ctx, user_id.0).await {
                        if let Err(e) = send_user_info(&user_info).await {
                            eprintln!("Failed to send user info: {:?}", e);
                        }
                        format!(
                            "# Authentication successful!\n- Profile link: https://axyl.wtf/{}",
                            user_info.username
                        )
                    } else {
                        "Authentication successful, but failed to fetch user info.".to_string()
                    }
                } else {
                    "Authentication failed.".to_string()
                };

                if let Err(why) = command
                    .create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| message.content(content))
                    })
                    .await
                {
                    println!("Failed to respond to slash command: {:?}", why);
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        ctx.set_presence(
            Some(Activity::playing("Testing")),
            OnlineStatus::DoNotDisturb,
        )
        .await;

        let guild_id = GuildId(
            std::env::var("GUILD_ID")
                .expect("Expected guild id in environment")
                .parse()
                .expect("Invalid guild id"),
        );
        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands.create_application_command(|command| {
                command
                    .name("authenticate")
                    .description("Authenticate with the bot")
            })
        })
        .await
        .expect("Failed to create command");
        println!("Created command: {:#?}", commands);
    }
}

pub async fn start_bot() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await?;

    client.start().await?;
    Ok(())
}
