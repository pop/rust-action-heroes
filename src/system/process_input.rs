use crate::lib::TransformedInputEvent;
use amethyst::{
    derive::SystemDesc,
    ecs::{Read, System, SystemData, Write},
    input::{InputHandler, StringBindings, VirtualKeyCode},
    shrev::EventChannel,
};
use std::{collections::BTreeSet, iter::FromIterator};

///
/// ...
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

    ///
    /// ...
    ///
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
            Some(m) => {
                input_event_channel.single_write(m)
            }
            None => (),
        };

        // We track which keys are currently pressed,
        // To avoid spamming "UP UP UP UP UP", etc.
        self.curr = BTreeSet::<VirtualKeyCode>::from_iter(input.keys_that_are_down());
    }
}
