use reqwest::Error;
use serde::Serialize;
use serenity::model::user::User;

#[derive(Serialize)]
pub struct UserInfo {
    pub id: u64,
    pub username: String,
    pub avatar_url: Option<String>,
}

impl From<&User> for UserInfo {
    fn from(user: &User) -> Self {
        UserInfo {
            id: user.id.0,
            username: user.name.clone(),
            avatar_url: user.avatar_url(),
        }
    }
}

pub async fn fetch_user_info(ctx: &serenity::prelude::Context, user_id: u64) -> Option<UserInfo> {
    if let Ok(user) = ctx.http.get_user(user_id).await {
        Some(UserInfo::from(&user))
    } else {
        None
    }
}

pub async fn send_user_info(user_info: &UserInfo) -> Result<(), Error> {
    let client = reqwest::Client::new();
    client
        .post("http://localhost:3000/api/user")
        .json(user_info)
        .send()
        .await?;
    Ok(())
}
