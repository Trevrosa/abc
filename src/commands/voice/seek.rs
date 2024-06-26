use std::time::Duration;

use serenity::all::{Context, Message};

use crate::utils::context::Ext;
use crate::TrackHandleKey;

pub async fn seek(ctx: &Context, msg: &Message) -> Result<(), &'static str> {
    let args: Vec<&str> = msg.content.trim().split(' ').collect();
    if args.len() != 2 {
        return Err("u dont say wat i seek to");
    }

    let Ok(to_seek) = args[1].parse() else {
        return Err("not number");
    };

    let global = ctx.data.try_read().unwrap();

    if global.contains_key::<TrackHandleKey>() {
        let Some(track) = global.get::<TrackHandleKey>() else {
            return Err("song ended..");
        };

        let seek = track.seek_async(Duration::from_secs(to_seek)).await;
        drop(global);

        if seek.is_ok() {
            ctx.reply(format!("seekd to {to_seek} secs"), msg).await;
        } else {
            ctx.reply("faild to seek", msg).await;
        }
    } else {
        ctx.reply("im not play anything", msg).await;
    }

    Ok(())
}
