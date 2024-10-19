use crate::auth::authenticate_user;
use crate::user_info::{fetch_user_info, send_user_info};
use serenity::all::{
    ActivityData, CreateCommand, CreateInteractionResponse, CreateInteractionResponseMessage,
    Interaction,
};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::user::OnlineStatus;
use serenity::prelude::*;

struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            if command.data.name == "authenticate" {
                let user_id = command.user.id;
                let content = if authenticate_user(&ctx, user_id).await {
                    if let Some(user_info) = fetch_user_info(&ctx, user_id).await {
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
                    .create_response(
                        &ctx.http,
                        CreateInteractionResponse::Message(
                            CreateInteractionResponseMessage::new().content(content),
                        ),
                    )
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
            Some(ActivityData::playing("Testing")),
            OnlineStatus::DoNotDisturb,
        );

        let guild_id = GuildId::new(
            std::env::var("GUILD_ID")
                .expect("Expected guild id in environment")
                .parse()
                .expect("Invalid guild id"),
        );

        let commands = GuildId::create_command(
            guild_id,
            &ctx.http,
            CreateCommand::new("authenticate").description("Authenticate with the bot"),
        )
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
