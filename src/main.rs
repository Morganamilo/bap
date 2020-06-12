const TOKEN: &str = env!("BAP");

use serenity::model::prelude::*;
use serenity::prelude::*;

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, ctx: Context, data: Ready) {
        println!("{} is connected!", data.user.tag());
        ctx.set_activity(Activity::playing("Bap"));
    }

    fn message(&self, ctx: Context, message: Message) {
        if !message.is_own(&ctx)
            && message
                .content
                .replace(char::is_whitespace, "")
                .to_lowercase()
                .contains("bap")
        {
            bap(message, &ctx);
        }
    }
}

fn bap(message: Message, ctx: &Context) {
    if message.mention_everyone {
        if let Some(guild) = message.guild(ctx) {
            let mut msg = String::new();
            for user in guild.read().members.values() {
                msg.push_str(&user.mention());
                msg.push_str(": Bap\n");
            }

            if let Err(err) = message.channel_id.send_message(ctx, |m| m.content(msg)) {
                println!("error sending message {:?}", err)
            }
        }
    } else if message.mentions.is_empty() {
        if let Err(err) = message.reply(ctx, "Bap") {
            println!("error sending message: {:?}", err)
        }
    } else {
        let mut msg = String::new();

        for user in &message.mentions {
            msg.push_str(&user.mention());
            msg.push_str(": Bap\n");
        }

        if let Err(err) = message.channel_id.send_message(ctx, |m| m.content(msg)) {
            println!("error sending message: {:?}", err)
        }
    }
}

fn main() {
    let mut client = Client::new(TOKEN, Handler).expect("Error creating client");

    if let Err(err) = client.start() {
        println!("runtime error: {:?}", err);
    }
}
