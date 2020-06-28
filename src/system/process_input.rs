//!
//! # Nobody has time to process raw keyboard input
//!
//! The only thing here is the ProcessInputSystem struct, so go read about that!
//!

use crate::lib::TransformedInputEvent;
use amethyst::{
    derive::SystemDesc,
    ecs::{Read, System, SystemData, Write},
    input::{InputHandler, StringBindings, VirtualKeyCode},
    shrev::EventChannel,
};
use std::{collections::BTreeSet, iter::FromIterator};

///
/// ProcessInputSystem transforms raw(ish) InputHandler inputs into a small set of possible
/// actions.
/// i.e., instead of all systems dealing with user input having to match on all possible keybaord
/// inputs, we match on "Move Up", "Move Down", "Interact", etc.
///
/// ProcessInputSystem also handles the "spamming input problem" in turn-based games by remembering
/// what the current action is and only sending a new signal when the user input changes.
/// This means that if someone pressed "Up", the game only sends/reads one "Move Up" message, even
/// though the InputHandler is spamming "Up" once per frame.
///
/// The "spamming input problem" could be solved by having some "turn" construct where we read
/// input for a turn, then reject input until the next turn, but this was easier.
/// The user is in control of when their turns are, and in this simple game there is no AI to take
/// a different turn, so the user's turns can happen as fast as they can press keys.
///
#[derive(SystemDesc)]
pub(crate) struct ProcessInputSystem {
    curr: BTreeSet<VirtualKeyCode>,
}

impl ProcessInputSystem {
    pub(crate) fn new() -> Self {
        ProcessInputSystem {
            curr: BTreeSet::<VirtualKeyCode>::new(),
        }
    }
}

impl<'s> System<'s> for ProcessInputSystem {
    type SystemData = (
        Read<'s, InputHandler<StringBindings>>,
        Write<'s, EventChannel<TransformedInputEvent>>,
    );

    fn run(&mut self, (input, mut input_event_channel): Self::SystemData) {
        // Figure out which movement the user is requesting
        let movement =
            if !self.curr.contains(&VirtualKeyCode::Up) && input.key_is_down(VirtualKeyCode::Up) {
                Some(TransformedInputEvent::Up)
            } else if !self.curr.contains(&VirtualKeyCode::Down)
                && input.key_is_down(VirtualKeyCode::Down)
            {
                Some(TransformedInputEvent::Down)
            } else if !self.curr.contains(&VirtualKeyCode::Left)
                && input.key_is_down(VirtualKeyCode::Left)
            {
                Some(TransformedInputEvent::Left)
            } else if !self.curr.contains(&VirtualKeyCode::Right)
                && input.key_is_down(VirtualKeyCode::Right)
            {
                Some(TransformedInputEvent::Right)
            } else if !self.curr.contains(&VirtualKeyCode::Space)
                && input.key_is_down(VirtualKeyCode::Space)
            {
                Some(TransformedInputEvent::Interact)
            } else {
                None
            };

        // Send the message for the movement service to listen
        match movement {
            Some(m) => input_event_channel.single_write(m),
            None => (),
        };

        // We track which keys are currently pressed,
        // To avoid spamming "UP UP UP UP UP", etc.
        self.curr = BTreeSet::<VirtualKeyCode>::from_iter(input.keys_that_are_down());
    }
}
