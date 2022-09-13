#![cfg_attr(not(feature = "std"), no_std)]
pub mod models;
pub mod traits;

pub use models::{Collective, ConvictionType};
pub use traits::CollectiveInspect;
