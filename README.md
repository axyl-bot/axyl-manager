# authbot
Discord bot for authentication & profile sharing between websites

## usage
- Export the bot-token as **`DISCORD_TOKEN`** & the guild-id as **`GUILD_ID`** and then run the commands below:
```bash
cargo run -- test_server # locally hosted website (testing.rs)
cargo run # bot
```
### custom website
- You'll have to swap out the URLs in **`user_info.rs`** & **`testing.rs`**, and then follow the instructions above
