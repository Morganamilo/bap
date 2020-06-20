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
        let count = has_bap(&message.content);
        if !message.is_own(&ctx) && count > 0 {
            if message.mention_everyone {
                if let Some(guild) = message.guild(&ctx) {
                    let users = guild.read();
                    let users = users.members.values().map(|v| v.user.read().clone());
                    bap(&ctx, &message, users, count);
                }
            } else if message.mentions.is_empty() {
                bap(&ctx, &message, iter::once(message.author.clone()), count);
            } else {
                bap(&ctx, &message, message.mentions.iter().cloned(), count);
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

fn has_bap(s: &str) -> usize {
    s.replace(char::is_whitespace, "")
        .to_lowercase()
        .chars()
        .bap()
        .collect::<String>()
        .matches("bap")
        .count()
}

fn bap(ctx: &Context, message: &Message, mentions: impl Iterator<Item = User>, count: usize) {
    let mut msg = String::new();

    for user in mentions {
        msg.push_str(&user.mention());
        msg.push_str(":");
        for _ in 0..count {
            msg.push_str(" Bap");
        }
        msg.push_str("\n");
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

struct Bap<I: Iterator> {
    iter: I,
    last: Option<I::Item>,
}

impl<I: Iterator> Iterator for Bap<I>
where
    I::Item: PartialEq + Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        while let Some(cur) = self.iter.next() {
            if Some(&cur) != self.last.as_ref() {
                self.last = Some(cur.clone());
                return Some(cur);
            }
        }

        None
    }
}

trait Bappable<I: Iterator> {
    fn bap(self) -> Bap<I>;
}

impl<I: Iterator> Bappable<I> for I {
    fn bap(self) -> Bap<I> {
        Bap {
            iter: self,
            last: None,
        }
    }
}
