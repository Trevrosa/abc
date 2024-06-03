# discord bot written in rust

## command structure
each module in `src/commands` must have a function with this signature:

```rust
fn command(ctx: serenity::client::Context, msg: serenity::model::channel::Message) -> impl Future<Output = ()>
```

or be a module that re-exports other modules.

*note that new modules in `src/commands` must be reflected in `crate::handlers::CommandHandler` manually.*

## building
- create file `token` at project root and put your bot token in
- create file `cat_apikey` at project root and put your [cat api key](https://thecatapi.com) in
- `cargo build -r`

## notes
- be careful using `ctx.data.read()`: it can deadlock since `ctx.data` is an instance of `Arc<RwLock<..>>`. a way to make these locks more clear is to use `ctx.data.try_read()`, and handle its result accordingly.
- `crate::utils::context::Ext` includes extension methods to `serenity::client::Context`
- `crate::serenity_ctrlc` is taken mostly from [yehuthi/serenity_ctrlc](https://github.com/yehuthi/serenity_ctrlc/) (thanks!). i have changed it to make it work with serenity 0.12.1, and also provide `ctx.data` to handlers.
- `clippy.toml` includes "forbidden" methods. these are only checked for when using `clippy` instead of `cargo check`
- `Cross.toml` is used by [cross](https://github.com/cross-rs/cross/) to cross-compile
- `.cargo/config.toml` makes cargo use clang to link. this makes it faster to compile, but can be omitted.
