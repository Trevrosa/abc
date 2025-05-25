pub mod prefix;
pub mod slash;

use serenity::all::Context;

use crate::{
    commands,
    utils::{reply::Replyer, Args},
};

#[inline]
pub(super) async fn handle_cmd(
    cmd: &str,
    ctx: &Context,
    replyer: &Replyer<'_>,
    args: Args<'_>,
) -> Result<(), &'static str> {
    match cmd {
        // misc commands
        "test" => commands::test(ctx, replyer).await,
        "cat" => commands::cat(ctx, replyer).await,
        "black" => commands::blacklist(ctx, replyer, args).await,

        "join" => commands::join(ctx, replyer, args).await,
        "leave" => commands::leave(ctx, replyer).await,

        "snipe" => commands::snipe(ctx, replyer).await,
        "editsnipe" => commands::edit_snipe(ctx, replyer).await,

        "getsong" => commands::get_song(ctx, replyer, args).await,

        // voice commands
        "play" => commands::voice::play(ctx, replyer, args).await,
        "pause" => commands::voice::pause(ctx, replyer).await,
        "resume" | "unpause" => commands::voice::resume(ctx, replyer).await,
        "status" => commands::voice::status(ctx, replyer).await,
        "loop" => commands::voice::set_loop(ctx, replyer).await,
        "stop" => commands::voice::stop(ctx, replyer).await,
        "seek" => commands::voice::seek(ctx, replyer, args).await,

        // do nothing if not matched
        &_ => Ok(()),
    }
}
