#[macro_use] extern crate serenity;
extern crate yaml_rust;

use serenity::prelude::*;
use serenity::model::*;

use yaml_rust::{YamlLoader,YamlEmitter};

use std::env;
use std::fs::File;
use std::io::prelude::*;

struct Handler;

impl EventHandler for Handler {
    // Set a handler for the 'on_message' event - so that whenever a new message is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through multi-threading, and so multiple of a single event can be dispatched simultaneously.
    fn on_message(&self, _: Context, msg: Message) {
        if msg.content == ".ping" {
            // Sending a message can fail, due to a network error, an authentication error, or lack of permissions to post in the channel, so log to stdout when some error happens, with a description of it.
            if let Err(why) = msg.channel_id.say("pong") {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    // Set a handler to be called on the 'on_ready' event. This is called when a shard is booted, and a READY payload is sent by Discord. This payload contains data like the current user's guild Ids, current user data, private channels, and more.
    //
    // In this case, just print what the current user's username is.
    fn on_ready (&self, _: Context, ready: Ready) {
        println!("{} is connected.", ready.user.name);
    }
}

fn main() {
    // Read config file (YAML)
//    println!("Reading config from file: {}", configfile);

    let mut config = File::open("config.yaml").expect("config.yaml not found.");

    let mut configcontents = String::new();
    config.read_to_string(&mut configcontents)
        .expect("Something went wrong while reading the file.");

    let docs = YamlLoader::load_from_str(&configcontents).unwrap();

    println!("{:?}", docs);

    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    // Create a new instance of the Client, logging in as a bot. This will automatically prepend your bot token with "Bot ", which is a requirement by discord for bot users.
    let mut client = Client::new(&token, Handler);

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform exponential backoff until it reconnects.
    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
