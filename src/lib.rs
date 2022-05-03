#![no_std]
//! AtomicU64/AtomicU32 wrappers for equally sized types implemented Copy

#[cfg(feature = "u64")]
mod subatomic64;
#[cfg(feature = "u64")]
pub use subatomic64::Subatomic64;

#[cfg(feature = "u32")]
mod subatomic32;
#[cfg(feature = "u32")]
pub use subatomic32::Subatomic32;
