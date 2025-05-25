# discord bot written in rust

## command structure
each module in `src/commands` must have a function with this signature:

```rust
async fn command(ctx: &Context, replyer: &Replyer/*, args: &[Arg]*/) -> Result<(), &'static str>
```
(where `Context` is `serenity::client::Context`, `Message` is `utils::reply::Replyer`, `Arg` is `utils::arg::Arg`.)
The commented part is optional.

and, to support slash commands, contain the function:

```rust
fn register() -> CreateCommand
```

(where `CreateCommand` is `serenity::builder::CreateCommand`)

or be a module that re-exports other modules that conform to above.

*note that new modules in `src/commands` must be reflected in `crate::handlers::CommandHandler` manually.*

## building
- install [songbird's](https://github.com/serenity-rs/songbird?tab=readme-ov-file#dependencies) dependencies (`apt install build-essential autoconf automake libtool m4`) 
- install [mold](https://github.com/rui314/mold)
- create file `token` at project root and put your bot token in
- create file `cat_apikey` at project root and put your [cat api key](https://thecatapi.com) in
- for the command `getsong`:
  - if you have a folder you want to put files larger than 10mb, set the `ABC_SHARED_DIR` environment variable to that folder.
  - if you have an external host url, create file `external_host` and put the base url in. if you don't, just `touch external_host`
  - get your [spotify oauth credentials](https://developer.spotify.com/dashboard), put your client_id and secret in files `spotify_clientid` and `spotify_secret` respectively.
  - get your [youtube oauth credentials](https://developers.google.com/youtube/v3/guides/auth/devices#prerequisites), put your client_id and secret in files `yt_clientid` and `yt_secret` respectively.
- install [yt-dlp](https://github.com/yt-dlp/yt-dlp/) and make sure the binary is available in `/usr/bin`
- if on windows, building might not work.
- `cargo build -r`

## notes
- be careful using `ctx.data.read()`: it can deadlock since `ctx.data` is an instance of `Arc<RwLock<..>>`. a way to make these locks more clear is to use `ctx.data.try_read()`, and handle its result accordingly.
- `crate::utils::context::Ext` includes extension methods to `serenity::client::Context`
- `crate::serenity_ctrlc` is taken mostly from [yehuthi/serenity_ctrlc](https://github.com/yehuthi/serenity_ctrlc/) (thanks!). i have changed it to make it work with serenity 0.12.1, and also provide `ctx.data` to handlers.
- `clippy.toml` includes "forbidden" methods. these are only checked for when using `clippy` instead of `cargo check`
- `Cross.toml` is used by [cross](https://github.com/cross-rs/cross/) to cross-compile
- `.cargo/config.toml` makes cargo use clang to link. this makes it faster to compile, but can be omitted.
