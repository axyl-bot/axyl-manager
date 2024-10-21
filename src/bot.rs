use crate::{
    auth::authenticate_user,
    user_info::{fetch_user_info, send_user_info},
};
use serenity::{
    all::{
        ActivityData, Command, CreateCommand, CreateInteractionResponse,
        CreateInteractionResponseMessage, GatewayIntents, Interaction,
    },
    async_trait,
    model::{gateway::Ready, user::OnlineStatus},
    prelude::*,
};

struct Handler;

#[async_trait]
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
            Some(
                ActivityData::streaming("twitch.tv/axylprojects", "https://twitch.tv/axylprojects")
                    .expect("Failed to create streaming activity"),
            ),
            OnlineStatus::DoNotDisturb,
        );

        match Command::get_global_commands(&ctx.http).await {
            Ok(commands) => {
                for command in commands {
                    if let Err(why) = Command::delete_global_command(&ctx.http, command.id).await {
                        println!(
                            "Failed to delete global command {}: {:?}",
                            command.name, why
                        );
                    } else {
                        println!("Deleted global command: {}", command.name);
                    }
                }
            }
            Err(why) => println!("Failed to get global commands: {:?}", why),
        }

        println!("Deleted all existing global commands.");

        let commands =
            vec![CreateCommand::new("authenticate").description("Authenticate with the bot")];

        match Command::set_global_commands(&ctx.http, commands).await {
            Ok(cmds) => println!("Successfully registered {} global commands", cmds.len()),
            Err(why) => println!("Failed to register global commands: {:?}", why),
        }
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
