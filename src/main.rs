use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::Client;
use serenity::model::id::*;

struct Handler;

impl EventHandler for Handler {
    fn cache_ready(&self, _context: Context, guilds: Vec<GuildId>) {
        let num_guilds = guilds.len();
        println!("{:#?}", guilds);
        println!("Guilds in the Cache: {}", num_guilds);
    }
    fn message(&self, context: Context, msg: Message) {
        if msg.content == "!ping" {
            let _ = msg.channel_id.say(&context, "Pong!");
        }
    }
}

fn main() {
    let token = "NzY3NDUwNDY2NzE5MTA1MDU0.X4yF8Q.WwssW-Fs1zF0icwXvIDaywWh4jk";
    let mut client = Client::new(&token, Handler).unwrap();
    if let Err(err) = client.start() {
        println!("Failed to start client: {:#?}", err);
    }
}
