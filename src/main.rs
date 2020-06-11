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
        match ctx.http.get_current_user() {
            Ok(user) => {
                if message.author.id != user.id
                    && message
                        .content
                        .replace(char::is_whitespace, "")
                        .to_lowercase()
                        .contains("bap")
                {
                    bap(message, &ctx);
                }
            }
            Err(err) => println!("error getting current user: {:?}", err),
        }
    }
}

fn bap(message: Message, ctx: &Context) {
    if message.mention_everyone {
        if let Some(guild) = message.guild(&ctx.cache) {
            let mut msg = String::new();
            for user in guild.read().members.values() {
                msg.push_str(&user.mention());
                msg.push_str(": Bap\n");
            }

            if let Some(channel) = message.channel(&ctx.cache) {
                if let Some(private) = channel.guild() {
                    if let Err(err) = private.read().send_message(&ctx.http, |m| m.content(msg)) {
                        println!("error sending message {:?}", err)
                    }
                }
            }
        }
    } else if message.mentions.is_empty() {
        if let Err(err) = message.reply(&ctx.http, "Bap") {
            println!("error sending message {:?}", err)
        }
    } else {
        let mut msg = String::new();

        for user in &message.mentions {
            msg.push_str(&user.mention());
            msg.push_str(": Bap\n");
        }

        if let Some(channel) = message.channel(&ctx.cache) {
            if let Some(private) = channel.guild() {
                if let Err(err) = private.read().send_message(&ctx.http, |m| m.content(msg)) {
                    println!("error sending message {:?}", err)
                }
            }
        }
    }
}

fn main() {
    let mut client = Client::new(TOKEN, Handler).expect("Error creating client");

    if let Err(err) = client.start() {
        println!("runtime error: {:?}", err);
    }
}
