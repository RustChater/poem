//! Some commonly used services that implement
//! [`Endpoint`](crate::endpoint::Endpoint).

mod files;
mod tower_compat;

pub use files::Files;
// pub use tower_compat::TowerCompat;
