const TOKEN: &str = env!("BAP");

use serenity::model::prelude::*;
use serenity::prelude::*;
use std::iter;

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
            if message.mention_everyone {
                if let Some(guild) = message.guild(&ctx) {
                    let users = guild.read();
                    let users = users.members.values().map(|v| v.user.read().clone());
                    bap(&ctx, &message, users);
                }
            } else if message.mentions.is_empty() {
                bap(&ctx, &message, iter::once(message.author.clone()));
            } else {
                bap(&ctx, &message, message.mentions.iter().cloned());
            }

            react(&ctx, &message, ReactionType::Unicode("🇧".into()));
            react(&ctx, &message, ReactionType::Unicode("🇦".into()));
            react(&ctx, &message, ReactionType::Unicode("🇵".into()));
        }
    }
}

fn react(ctx: &Context, message: &Message, reaction: ReactionType) {
    if let Err(err) = message.react(ctx, reaction) {
        println!("failed to react: {:?}", err);
    }
}

fn bap(ctx: &Context, message: &Message, mentions: impl Iterator<Item = User>) {
    let mut msg = String::new();

    for user in mentions {
        msg.push_str(&user.mention());
        msg.push_str(": Bap\n");
    }

    if let Err(err) = message.channel_id.send_message(ctx, |m| m.content(msg)) {
        println!("error sending message: {:?}", err)
    }
}

fn main() {
    let mut client = Client::new(TOKEN, Handler).expect("Error creating client");

    if let Err(err) = client.start() {
        println!("runtime error: {:?}", err);
    }
}
