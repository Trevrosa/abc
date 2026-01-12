//! This crate is required so both `macros` and `abc` can depend on the same stuff.

#![feature(type_alias_impl_trait)]

use serenity::{
    all::{Context, CreateCommand},
    prelude::TypeMapKey,
};
use songbird::tracks::TrackHandle;
use utils::{Args, Replyer};

pub type RunResult = impl Future<Output = Result<(), &'static str>>;

pub type RunFn = fn(&Context, &Replyer) -> RunResult;
pub type RunArgsFn = fn(&Context, &Replyer, Args) -> RunResult;

async fn a() -> Result<(), &'static str> {
    Ok(())
}

fn b() -> RunResult {
    return a();
}

/// A bot command.
#[derive(Debug)]
pub struct BotCommand {
    pub name: &'static str,
    pub has_args: bool,
    pub register: fn() -> CreateCommand,
    run: Option<RunFn>,
    run_args: Option<RunArgsFn>,
}

impl BotCommand {
    pub const fn new(
        name: &'static str,
        has_args: bool,
        register: fn() -> CreateCommand,
        run: Option<RunFn>,
        run_args: Option<RunArgsFn>,
    ) -> Self {
        Self {
            name,
            has_args,
            register,
            run,
            run_args,
        }
    }
}

inventory::collect!(BotCommand);

pub struct TrackHandleKey;

impl TypeMapKey for TrackHandleKey {
    type Value = TrackHandle;
}
