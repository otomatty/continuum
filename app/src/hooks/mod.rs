pub mod use_auth;

#[cfg(test)]
mod tests;

pub use use_auth::*;

// Re-export Session for use in shell function
#[cfg(feature = "ssr")]
pub use use_auth::Session;
