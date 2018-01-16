use std::collections::HashMap;
use std::fmt::Write;
use typemap::Key;

pub struct CommandCounter;

impl Key for CommandCounter {
    type Value = HashMap<String, u64>;
}

command!(command_counter(ctx, msg, _args){
    let mut contents = "Commands used:\n".to_string();

    let data = ctx.data.lock();
    let counter = data.get::<CommandCounter>().unwrap();

    for (k, v) in counter {
        let _ = write!(contents, "- {name}: {amount}\n", name=k, amount=v);
    }

    if let Err(why) = msg.channel_id.say(&contents) {
        println!("Error sending message: {:?}", why);
    }
});

command!(ping(_ctx, msg) {
    let _ = msg.channel_id.say("Pong");
});
