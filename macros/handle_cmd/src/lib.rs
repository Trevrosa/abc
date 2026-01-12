extern crate commands;

use glue::BotCommand;
use proc_macro::TokenStream;
use quote::quote;

/// Generates a match expression for commands registered via the `command` attribute.
#[proc_macro]
pub fn make_handle_cmds(_: TokenStream) -> TokenStream {
    let command_arms = inventory::iter::<BotCommand>
        .into_iter()
        .map(|command| {
            let name = &command.name;
            if command.has_args {
                quote! { #name => commands::#name(ctx, replyer, args).await, }
            } else {
                quote! { #name => commands::#name(ctx, replyer).await, }
            }
        })
        .collect::<Vec<_>>();

    let output = quote! {
        #[inline]
        pub(super) async fn handle_cmd(
            cmd: &str,
            ctx: &Context,
            replyer: &Replyer<'_>,
            args: Args<'_>,
        ) -> Result<(), &'static str> {
            match cmd {
                #(#command_arms)*
                _ => Ok(()),
            }
        }
    };

    output.into()
}
