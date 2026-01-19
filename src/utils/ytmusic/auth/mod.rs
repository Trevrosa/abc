pub mod browser;
#[allow(unused)] // oauth cant be used for now
pub mod oauth;

/// An authentication type.
pub trait Authentication {
    /// The authentication header value.
    fn value(&self) -> String;
}
