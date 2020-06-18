/// # Switch System
///
/// Switches 
use amethyst::{
    derive::SystemDesc,
    ecs::{Join, ReadStorage, System, SystemData, Write},
    shrev::{EventChannel},
};

use crate::component::{Position, Movable, Switch};

pub(crate) enum SwitchEvent {
    AllSwitchesDown,
}

#[derive(SystemDesc, Default)]
pub(crate) struct SwitchSystem;

impl<'s> System<'s> for SwitchSystem {
    type SystemData = (
        ReadStorage<'s, Position>,
        ReadStorage<'s, Movable>,
        ReadStorage<'s, Switch>,
        Write<'s, EventChannel<SwitchEvent>>,
    );

    fn run(&mut self, (positions, movables, switches, mut channel): Self::SystemData) {
        let mut switch_down = 0;
        for (position_movable, _movable) in (&positions, &movables).join() {
            for (position_switch, _switch) in (&positions, &switches).join() {
                if position_movable == position_switch {
                    switch_down += 1;
                }
            }
        }
        for _switch in (&switches).join() {
            switch_down -= 1;
        }
        if switch_down == 0 {
            channel.single_write(SwitchEvent::AllSwitchesDown);
        }
    }
}
