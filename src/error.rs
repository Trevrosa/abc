use thiserror::Error;

#[derive(Error, Debug)]
pub enum Voice {
    #[error("VoiceClientNotInit")]
    VoiceClientNotInit,
    #[error("Handler")]
    Handler,
    #[error("Manager")]
    Manager,
}

#[derive(Error, Debug)]
pub enum PlayCommand {
    #[error("Download")]
    Download,
    #[error("NoMedia")]
    NoMedia,
}

#[derive(Error, Debug)]
pub enum General {
    #[error("CommandFailed")]
    CommandFailed,
    #[error("Argument")]
    Argument,
    #[error("DiscordGet")]
    DiscordGet,
    #[error("CommandRequirement")]
    CommandRequirement,
}
