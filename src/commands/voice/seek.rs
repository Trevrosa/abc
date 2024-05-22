use std::time::Duration;

use serenity::all::{Context, Message};

use super::Utils;
use crate::TrackHandleKey;

pub async fn seek(ctx: Context, msg: Message) {
    let args: Vec<&str> = msg.content.trim().split(' ').collect();
    if args.len() != 2 {
        ctx.reply("u dont say wat i seek to", &msg).await;
        return;
    }

    let Ok(to_seek) = args[1].parse() else {
        ctx.reply("not number", &msg).await;
        return;
    };

    let global = ctx.data.try_read().unwrap();

    if global.contains_key::<TrackHandleKey>() {
        let Some(track) = global.get::<TrackHandleKey>() else {
            ctx.reply("faild to pause", &msg).await;
            return;
        };

        if track.seek_async(Duration::from_secs(to_seek)).await.is_ok() {        
            ctx.reply(&format!("seekd to {to_seek} secs"), &msg).await;
        } else {        
            ctx.reply("faild to seek", &msg).await;
        }
    } else {
        ctx.reply("im not play anything", &msg).await;
    }
}
