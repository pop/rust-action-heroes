//!
//! # Systems
//!
//! Systems describe how entities with certain components interact.
//!
//! Take for example you have two entities:
//!
//! ```text
//! E1:
//!   Component::Squishy: Flag
//!   Component::Vulnerable: Bool
//!   ...
//! ```
//!
//! ```text
//! E2:
//!   Component::Squisher: Flag
//!   Component::SquishingMood: Bool
//!   ...
//! ```
//!
//! These two entities might interact via a `SquishSystem` which says:
//!
//! ```text
//! SquishSystem(
//!     (squishies, squishers, vulnerables, squishingmood): (Squishy, Squisher, Vulnerable, SquishingMood)
//! ):
//!     for (squishy, vulnerable) in (squshies, vulnerables):
//!         if vulnerable:
//!             for (squisher, squishingmood) in (squishers, squishingmoods):
//!                 if squishingmood:
//!                     squisher.squish(squishy)
//! ```
//!
//! This collects all entities with the `Squishy`, `Vulnerable`, `Squisher`, and `SquishingMood`
//! components and conveniently allows us to iterate over any slice of those four.
//!
//! In the above example we are iterating over all entities with _both_ the `squishy` and
//! `vulnerable` component, then if `vulnerable` is true, we iterate over all `squisher` and
//! `squishingmood` entities and do some action.
//!
//! ## Event Channels
//!
//! One pattern I use a lot is to have a system either _listening_ or _writing_ an Event Channel.
//! This means that systems are more interrupt driven as opposed to real-time.
//!
//! Events don't scale to all problems, even in this game we have some systems which are real-time,
//! but in a turn-based-like game events are very useful for minimizing unecessary compute and
//! reasoning about what a sytem will do.
//!

pub mod movement_solver;
pub(crate) use movement_solver::*;

pub mod mover;
pub(crate) use mover::*;

pub mod move_sound;
pub(crate) use move_sound::*;

pub mod process_input;
pub(crate) use process_input::*;

pub mod grid;
pub(crate) use grid::*;

pub mod grab;
pub(crate) use grab::*;

pub mod sprites;
pub(crate) use sprites::*;

pub mod level;
pub(crate) use level::*;

pub mod locks;
pub(crate) use locks::*;

pub mod switches;
pub(crate) use switches::*;

pub mod door;
pub(crate) use door::*;
