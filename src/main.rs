use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use dotenv::dotenv;

mod game_night;
use game_night::{GameNightConfig, format_game_night_status, format_next_game_night, is_game_night_now};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Called when a message is created
    async fn message(&self, ctx: Context, msg: Message) {
        // Ignore messages from the bot itself
        if msg.author.bot {
            return;
        }

        let content = msg.content.to_lowercase();
        
        match content.as_str() {
            "!ping" => {
                if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                    println!("Error sending message: {:?}", why);
                }
            }
            "!gamenight" => {
                let config = GameNightConfig::default();
                let status = format_game_night_status(&config);
                if let Err(why) = msg.channel_id.say(&ctx.http, status).await {
                    println!("Error sending message: {:?}", why);
                }
            }
            "!nextgame" => {
                let config = GameNightConfig::default();
                let next_game = format_next_game_night(&config);
                if let Err(why) = msg.channel_id.say(&ctx.http, next_game).await {
                    println!("Error sending message: {:?}", why);
                }
            }
            "!isgamenight" => {
                let config = GameNightConfig::default();
                let response = if is_game_night_now(&config) {
                    "Yes! Game night is happening now! 🎮"
                } else {
                    "No, it's not game night yet. Use !gamenight to see when the next one is."
                };
                if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
                    println!("Error sending message: {:?}", why);
                }
            }
            "!help" => {
                let help_text = "**Available Commands:**\n\
                    `!ping` - Test if bot is responsive\n\
                    `!gamenight` - Show game night status\n\
                    `!nextgame` - Show when the next game night is\n\
                    `!isgamenight` - Check if game night is happening now\n\
                    `!help` - Show this help message";
                
                if let Err(why) = msg.channel_id.say(&ctx.http, help_text).await {
                    println!("Error sending message: {:?}", why);
                }
            }
            _ => {}
        }
    }

    // Called when the bot is ready
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Load .env file
    dotenv().ok();
    
    // Get token from environment variable
    let token = std::env::var("DISCORD_TOKEN")
        .expect("Expected DISCORD_TOKEN in environment");
    
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Start listening for events
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
