//! Evo L476 adapters promoted from the verified L4 smoke tests (ADR-L4-012).
#![no_std]
#[cfg(feature = "firmware")]
pub mod capture;
pub mod capture_math;
#[cfg(feature = "firmware")]
pub mod dfsdm;
#[cfg(feature = "firmware")]
pub mod identity;
pub mod pcm;
#[cfg(feature = "firmware")]
pub mod sd;
#[cfg(feature = "firmware")]
pub mod stack_probe;
