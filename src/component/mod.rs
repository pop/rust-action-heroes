//!
//! Putting the C in ECS!
//!
//! Components are pieces of data attached to an entity which we reference in systems.
//! They allow us to think about systems in terms of "how do things with X interact with Y" instead
//! of "how does a X interact with a Y".
//!
//! Components are a huge part of data-driven design in games and are super powerful!
//!
//! Some components are just flags, like "This entity is a key", while others contain data and
//! methods, like "This has a position in space and can be moved in these directions".
//!
//! This game is relatively small, so I haven't seen much of this, but components also allow you to
//! design a game with interesting rules and allows you to "discover" emergent properties of those
//! how those rules interact.
//!

mod movable;
pub(crate) use movable::*;

mod immovable;
pub(crate) use immovable::*;

mod named;
pub(crate) use named::*;

mod holding;
pub(crate) use holding::*;

mod exit;
pub(crate) use exit::*;

mod position;
pub(crate) use position::*;

mod lock;
pub(crate) use lock::*;

mod key;
pub(crate) use key::*;

mod switch;
pub(crate) use switch::*;

mod door;
pub(crate) use door::*;
