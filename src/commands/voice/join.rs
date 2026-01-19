use serenity::all::{
    ChannelType, CommandOptionType, Context, CreateCommand, CreateCommandOption, InteractionContext,
};

use crate::utils::{context::CtxExt, reply::Replyer, ArgValue, Args};

pub async fn join(
    ctx: &Context,
    replyer: &Replyer<'_>,
    args: Args<'_>,
) -> Result<(), &'static str> {
    let Some(guild_id) = replyer.guild() else {
        return Err("faild to get guild");
    };

    let Ok(channels) = guild_id.channels(&ctx).await else {
        return Err("faild to get channels");
    };

    let mut channels = channels.iter();

    let channel = if let Some(ArgValue::Channel(channel)) = args.first().map(|a| &a.value) {
        let Ok(channel) = ctx.http.get_channel(channel.id).await else {
            return Err("channel not exist");
        };

        let channel = channel.guild().unwrap();

        if channel.kind == ChannelType::Voice {
            Some(channel)
        } else {
            None
        }
    } else {
        ctx.find_user_channel(replyer.user(), ChannelType::Voice, &mut channels)
            .cloned()
    };

    let Some(channel) = channel else {
        return Err("u arent in a vc");
    };

    if let Some(manager) = songbird::get(ctx).await.clone() {
        if manager.join(guild_id, channel.id).await.is_ok() {
            ctx.reply("joined u", replyer).await;
        } else {
            ctx.reply("faild to join", replyer).await;
        }
    } else {
        return Err("voice manager failed");
    }

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("join")
        .add_context(InteractionContext::Guild)
        .description("join the specified channel or the one you're currently in")
        .add_option(CreateCommandOption::new(
            CommandOptionType::Channel,
            "channel",
            "the channel the bot should join",
        ))
}
