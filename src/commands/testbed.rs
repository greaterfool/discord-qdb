command!(testbed(_ctx, msg, _args){
    if let Err(why) = msg.channel_id.send_message(|m| m
                                                  .content("Friday, January 12, 2018")
                                                  .embed(|e| e
                                                         .title("marmalade")
                                                         .description("*date*\n im gey"))) {
        println!("Err sending testbed: {:?}", why);
    }
});
