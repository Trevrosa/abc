use std::time::Instant;

use serenity::all::{
    Command, Context, CreateInteractionResponse, CreateInteractionResponseMessage, EventHandler,
    GuildId, Interaction, Ready,
};
use serenity::async_trait;
use tracing::{error, info};

use crate::commands::voice::{pause, play, resume, seek, set_loop, status, stop};
use crate::commands::{blacklist, cat, edit_snipe, get_song, join, leave, snipe, test};
use crate::handlers::command::handle_cmd;
use crate::utils::context::CtxExt;
use crate::utils::reply::Replyer;
use crate::utils::{Arg, Args};

pub struct SlashCommands;

#[allow(clippy::unreadable_literal)]
const TESTING_GUILD: u64 = 1131152701732954122;

// FIXME: use Replyer and figure out args?

#[async_trait]
impl EventHandler for SlashCommands {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let Interaction::Command(command) = interaction else {
            return;
        };

        println!("got slash cmd `{}`", command.data.name);

        // we have to tell discord we are in the progress of responding
        let initial_resp =
            CreateInteractionResponse::Defer(CreateInteractionResponseMessage::new());
        command
            .create_response(&ctx.http, initial_resp)
            .await
            .unwrap();

        let replyer = Replyer::Slash(&command);

        let parse_start = Instant::now();
        let args: Vec<Arg> = command
            .data
            .options()
            .into_iter()
            .filter_map(Arg::from_resolved)
            .collect();
        info!(
            "took {:?} to parse {} args from interaction",
            parse_start.elapsed(),
            args.len()
        );

        let result: Result<(), &str> =
            handle_cmd(&command.data.name, &ctx, &replyer, Args::new(&args)).await;

        if let Err(error) = result {
            // if error == "", no response
            if !error.is_empty() {
                ctx.reply(error, &replyer).await;
            }
        }
    }

    async fn ready(&self, ctx: Context, _: Ready) {
        let testing_guild = GuildId::new(TESTING_GUILD);

        let commands = vec![
            test::register(),
            play::register(),
            snipe::register(),
            leave::register(),
            join::register(),
            get_song::register(),
            edit_snipe::register(),
            cat::register(),
            blacklist::register(),
            stop::register(),
            status::register(),
            set_loop::register(),
            seek::register(),
            resume::register(),
            pause::register(),
        ];

        testing_guild
            .set_commands(&ctx.http, commands.clone())
            .await
            .expect("failed to register guild cmds");

        info!("finished setting testing slash cmds");

        info!("now registering global cmds, might take a while.");
        let register_start = Instant::now();
        for command in commands {
            if let Err(err) = Command::create_global_command(&ctx.http, command).await {
                error!("failed to set global command: {err:?}");
            }
        }
        info!(
            "finished registering global slash cmds (took {:?})",
            register_start.elapsed()
        );
    }
}
