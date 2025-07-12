//! Abstract design patterns for the Jupiter Design System
//!
//! This module defines reusable interaction patterns and semantic concepts
//! that can be applied across different component types.

pub mod actions;
pub mod button;
pub mod card;
pub mod focus;
pub mod interactions;

// Re-export commonly used patterns
pub use actions::*;
pub use button::*;
pub use card::*;
pub use focus::*;
pub use interactions::*;
