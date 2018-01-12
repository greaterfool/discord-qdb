extern crate serenity;
extern crate yaml_rust;
#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;
extern crate typemap;

use serenity::prelude::*;
use serenity::model::*;
use serenity::framework::standard::{Args, Command, DispatchError, StandardFramework, help_commands};

use yaml_rust::{YamlLoader,YamlEmitter};

use bson::Bson;
use mongodb::ThreadedClient;
use mongodb::db::ThreadedDatabase;

use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::env;
use std::fmt::Write;
use std::sync::Arc;
use typemap::Key;

struct CommandCounter;

impl Key for CommandCounter {
    type Value = HashMap<String, u64>;
}

struct Handler;

impl EventHandler for Handler {
    // Set a handler for the 'on_message' event - so that whenever a new message is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through multi-threading, and so multiple of a single event can be dispatched simultaneously.
    fn on_message(&self, _: Context, msg: Message) {
        // I'm gonna leave ping in here, as a debug kind of thing.
        if msg.content == ".ping" {
            if let Err(why) = msg.channel_id.say("pong") {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    // Set a handler to be called on the 'on_ready' event. This is called when a shard is booted, and a READY payload is sent by Discord.
    // This payload contains data like the current user's guild Ids, current user data, private channels, and more.
    //
    // In this case, just print what the current user's username is.
    fn on_ready (&self, _: Context, ready: Ready) {
        println!("{} is connected.", ready.user.name);
    }
}

fn main() {
    //  -- Read YAML Config file. --
    let mut config = File::open("config.yaml").expect("config.yaml not found.");
    println!("Reading config from file: {:?}", config);

    let mut configcontents = String::new();
    config.read_to_string(&mut configcontents)
        .expect("Something went wrong while reading the file.");

    let docs = YamlLoader::load_from_str(&configcontents).unwrap();
    let doc = &docs[0];

    println!("{:?}", docs); // Debug support

    let mut out_str = String::new(); {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(doc).unwrap();
    }
    println!("{}", out_str); // Debug support
    println!("{:?}", doc["Token"].as_str()); // Debug support

    let token = doc["Token"].as_str()
        .expect("Expected a token in config file.");

    let mdb_client = mongodb::Client::connect("localhost", 27017)
        .expect("Failed to initialize MongoDB standalone client.");
    let coll = mdb_client.db("test").collection("movies");
    let mdbdoc = doc! {
        "title": "Jaws",
        "array": [1, 2, 3],
    };

    coll.insert_one(mdbdoc.clone(), None)
        .ok().expect("Failed to insert document.");

    let mut cursor = coll.find(Some(mdbdoc.clone()), None)
        .ok().expect("Failed to execute find.");

    let item = cursor.next();

    match item {
        Some(Ok(mdbdoc)) => match mdbdoc.get("title") {
            Some(&Bson::String(ref title)) => println!("{}", title),
            _ => panic!("Expected title to be a string!"),
        },
        Some(Err(_)) => panic!("Failed to get next from server!"),
        None => panic!("Server returned to results!"),
    }

    // Create a new instance of the Client, logging in as a bot. This will automatically prepend your bot token with "Bot ", which is a requirement by discord for bot users.
    let mut client = serenity::Client::new(&token, Handler);

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform exponential backoff until it reconnects.
    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
