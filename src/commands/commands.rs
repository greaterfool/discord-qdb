command!(commands(ctx, msg, _args){
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
