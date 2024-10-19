use serenity::model::id::UserId;
use serenity::prelude::*;

pub async fn authenticate_user(ctx: &Context, user_id: UserId) -> bool {
    if let Ok(guilds) = ctx.http.get_guilds(None, None).await {
        if let Some(guild) = guilds.first() {
            ctx.http.get_member(guild.id, user_id).await.is_ok()
        } else {
            false
        }
    } else {
        false
    }
}
