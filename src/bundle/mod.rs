//!
//! Bundles are collections of resources, components, and systems that should be created together.
//!
//! I use Bundles to instantiate all systems which require EventChannels, so MovementBundle is
//! definitely overloaded.
//!
//! I couldn't just call it "RustActionHeroesBundle" because I have other systems.
//!
//! There is definitely room for improvement in the code organization here.
//!

mod movement;
pub(crate) use movement::*;
