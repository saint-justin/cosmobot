use cards::make_card_map;
use cards::parse_cards;
use serenity::async_trait;
use serenity::prelude::*;
use poise::serenity_prelude::{GatewayIntents, User};
use serenity::utils::MessageBuilder;

mod aws_helpers;
mod cards;

// #[group]
// #[commands(ping)]
// struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn search_for(
    ctx: Context<'_>,
    card: String,
) -> Result<(), Error> {
    let card_map = make_card_map(parse_cards()?);
    match card_map.get(&card) {
        Some(found_card) => {
            let response = MessageBuilder::new()
                .push("Card found!\n")
                .push(format!("Name:   {}\n", found_card.name))
                .push(format!("Power:  {}\n", found_card.power))
                .push(format!("Energy: {}", found_card.cost))
                .build();
            ctx.say(response).await?;
            Ok(())
        }
        None => {
            let response = format!("No cards found with name `{}`", card);
            ctx.say(response).await?;
            Ok(())
        }
    }
}

#[tokio::main]
async fn main() {
    // Bot token from Secrets Manager
    let token = aws_helpers::get_bot_token()
        .await
        .expect("Error getting bot API token");

        let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age(), search_for()],
            ..Default::default()
        })
        .token(token)
        .intents(GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });

    framework.run().await.unwrap();
    
    /*
    // Build serenity client with framework + gateway intents
    let framework = StandardFramework::new().group(&GENERAL_GROUP);
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    let parse_str = cards::parse_cards()?;

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
    */
}