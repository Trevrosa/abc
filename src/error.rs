use thiserror::Error;

#[derive(Error, Debug)]
pub enum VoiceError {
    #[error("VoiceClientNotInit")]
    VoiceClientNotInit,
    #[error("HandlerError")]
    HandlerError,
    #[error("ManagerError")]
    ManagerError,
}

#[derive(Error, Debug)]
pub enum PlayError {
    #[error("DownloadError")]
    DownloadError,
    #[error("NoMediaError")]
    NoMediaError,
}

#[derive(Error, Debug)]
pub enum GeneralError {
    #[error("CommandFailed")]
    CommandFailed,
    #[error("ArgumentError")]
    ArgumentError,
    #[error("DiscordGetError")]
    DiscordGetError,
    #[error("CommandRequirementError")]
    CommandRequirementError,
}
