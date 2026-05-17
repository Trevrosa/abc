use serenity::all::{
    CommandOptionType, Context, CreateCommand, CreateCommandOption, InteractionContext,
};

use crate::Volume;
use crate::utils::context::CtxExt;
use crate::utils::reply::Replyer;
use crate::utils::{ArgValue, Args};

pub async fn set_volume(
    ctx: &Context,
    replyer: &Replyer<'_>,
    args: Args<'_>,
) -> Result<(), &'static str> {
    let Some(guild_id) = replyer.guild() else {
        return Err("u not in a guild");
    };

    let Some(ArgValue::Integer(volume)) = args.first_value() else {
        return Err("u no say what volume to set");
    };

    if *volume < 0 {
        return Err("volume cant be negative");
    }

    #[allow(clippy::cast_precision_loss)]
    let volume = *volume as f32 / 100.0;

    if let Some(track) = ctx.current_track(guild_id).await {
        track.set_volume(volume).unwrap();
    }

    {
        let mut data = ctx.data.write().await;
        data.get_mut::<Volume>().unwrap().insert(guild_id, volume);
    }

    ctx.reply("volume set!", replyer).await;

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("volume")
        .description("set the volume for all tracks in this server")
        .add_context(InteractionContext::Guild)
        .add_option(CreateCommandOption::new(
            CommandOptionType::Integer,
            "value",
            "must be positive, in percentage",
        ))
}
