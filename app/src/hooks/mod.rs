pub mod use_auth;

#[cfg(target_arch = "wasm32")]
mod use_auth_client;

#[cfg(test)]
mod tests;

pub use use_auth::*;

// Re-export client-side functions
#[cfg(target_arch = "wasm32")]
pub use use_auth_client::*;

// Re-export Session for use in shell function
#[cfg(feature = "ssr")]
pub use use_auth::Session;
