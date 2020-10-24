pub mod config;
use config::Config;

use serenity::model::id::GuildId;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::Client;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng,};

struct Handler;

// for each guild, if chores channel doesn't exist, create it
// start working and sending messages to chores channel

static CHORES_CHANNEL_NAME: &str = "chores";

fn get_chores_channel(context: &Context, guild_id: &GuildId) -> std::result::Result<GuildChannel, ()> {
    let channels_result = guild_id.channels(context);
    if channels_result.is_err() {
        return Err(());
    }
    for (_, channel) in channels_result.unwrap() {
        if channel.name() == CHORES_CHANNEL_NAME {
            println!("found chores channel!");
            return Ok(channel);
        }
    }
    return Err(());
}

fn ensure_chores_channel_exists(context: &Context, guild_id: &GuildId) {
    let chores_channel_result = get_chores_channel(context, guild_id);
    if chores_channel_result.is_err() {
        let _ = guild_id.create_channel(context, |channel| channel.name(CHORES_CHANNEL_NAME).kind(ChannelType::Text)).unwrap();


    }
}


fn say_hello(context: &Context, guild_id: &GuildId) {   
   let chores_channel = get_chores_channel(context, guild_id).unwrap();
   for member_result in  guild_id.members_iter(context) {
       let member = member_result.unwrap();
       println!("member is {}", member.display_name());
       let _ = chores_channel.say(context, format!("Hello, {}!", member.mention()));
   }
}

fn assign_chores(context: &Context, guild_id: &GuildId) {
    let chores_channel = get_chores_channel(context, guild_id).unwrap();
    let mut chores: Vec<&str> = vec! [
        "Floors",
        "Kitchen",
        "Sink",
        "Dishes",
        "Four-season",
    ];

    let mut rng = thread_rng();
    chores.shuffle(&mut rng);
    for chore in chores.iter() {
        let _ = chores_channel.say(context, format!("Chore is {}", chore));
    }
}

impl EventHandler for Handler {
    fn cache_ready(&self, context: Context, guild_ids: Vec<GuildId>) {
        for guild_id in guild_ids {
            ensure_chores_channel_exists(&context, &guild_id);
            assign_chores(&context, &guild_id);
            say_hello(&context, &guild_id);
        }

    }
    fn message(&self, context: Context, msg: Message) {

        let responses: Vec<&str> = vec![
            "What is earned with hard labor is eaten with pleasure.",
            "The hardest work of all is to do nothing.",
            "It is hard work to be the mother of many pigs.",
            "It is no credit to anyone to work too hard.",
            "Hard work beats all the tonics and vitamins in the world.",
            "Hard work is the key to success",
            "Whatever muscles I have are the product of my own hard work.",
            "You don't get anything clean without getting something else dirty.",
            "Housework, if you do it right, will kill you.",
            "There is no need to do any housework at all. After the first four years the dirt doesn't get any worse.",
            "A spotless house is the sign of a misspent life.",
        ];
        let mut rng = thread_rng();
        let n: usize = rng.gen_range(0, responses.len());

        let response = responses[n];
        if msg.content == "!Complete" || msg.content == "!complete" {
            let _ = msg.channel_id.say(&context, response);
        }
    }
}

fn main() {
    let token = Config::new().token();
    let mut client = Client::new(&token, Handler).unwrap();
    if let Err(err) = client.start() {
        println!("Failed to start client: {:#?}", err);
    }
}
