extern crate amethyst_test;

use amethyst::{
    core::{bundle::SystemBundle, transform::TransformBundle, SystemDesc},
    derive::SystemDesc,
    ecs::{DispatcherBuilder, Read, System, SystemData, World, Write, WriteExpect, prelude::{Component, DenseVecStorage}},
    input::{InputBundle, InputHandler, StringBindings, VirtualKeyCode},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    shrev::{EventChannel, ReaderId},
    utils::application_root_dir,
    Error,
};
use std::{
    collections::BTreeSet,
    env,
    iter::FromIterator,
};
use core::result::Result;

///
/// ...
///
struct GameState;

///
/// ...
///
impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        println!(";;; Creating Movable entitiy");

        world
            .create_entity()
            .with(Movable::new())
            .build();
    }
}

///
/// Input Events
///
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum MyInputEvent {
    Up,
    Down,
    Left,
    Right,
    Interact,
}

///
/// MyInputSystem struct
///
#[derive(SystemDesc)]
pub struct MyInputSystem {
    curr: BTreeSet<VirtualKeyCode>,
}

impl MyInputSystem {
    fn new() -> Self {
        MyInputSystem {
            curr: BTreeSet::<VirtualKeyCode>::new(),
        }
    }
}

///
/// MyInputSystem system
///
impl<'a> System<'a> for MyInputSystem {
    type SystemData = (
        Read<'a, InputHandler<StringBindings>>,
        Write<'a, EventChannel<MyInputEvent>>,
    );

    ///
    /// ...
    ///
    fn run(&mut self, (input, mut input_event_channel): Self::SystemData) {
        // Figure out which movement the user is requesting
        let movement =
            if !self.curr.contains(&VirtualKeyCode::Up)
                && input.key_is_down(VirtualKeyCode::Up)
            {
                Some(MyInputEvent::Up)
            } else if !self.curr.contains(&VirtualKeyCode::Down)
                && input.key_is_down(VirtualKeyCode::Down)
            {
                Some(MyInputEvent::Down)
            } else if !self.curr.contains(&VirtualKeyCode::Left)
                && input.key_is_down(VirtualKeyCode::Left)
            {
                Some(MyInputEvent::Left)
            } else if !self.curr.contains(&VirtualKeyCode::Right)
                && input.key_is_down(VirtualKeyCode::Right)
            {
                Some(MyInputEvent::Right)
            } else if !self.curr.contains(&VirtualKeyCode::Space)
                && input.key_is_down(VirtualKeyCode::Space)
            {
                Some(MyInputEvent::Interact)
            } else {
                None
            };

        // Send the message for the movement service to listen
        match movement {
            Some(m) => {
                input_event_channel.single_write(m)
            },
            None => (),
        };

        // We track which keys are currently pressed,
        // To avoid spamming "UP UP UP UP UP", etc.
        self.curr = BTreeSet::<VirtualKeyCode>::from_iter(input.keys_that_are_down());
    }
}

///
/// ...
///
#[cfg(test)]
mod tests {
    use super::*;
    use amethyst_test::prelude::*;
    use amethyst::winit::{
        DeviceId, ElementState, Event, KeyboardInput, ModifiersState, ScanCode, WindowEvent, WindowId,
    };
    use amethyst::input::InputEvent;
    use amethyst::input::StringBindings;

    const HIDPI: f32 = 1.0;

    #[test]
    /// This test verifies that MyInputSystem listens for user input and generates custom
    /// MyInputEvents.
    ///
    /// Each key press should generate one message.
    ///
    /// Each subsequent "key is pressed" message should not generate additional MyInputEvents.
    fn test_my_input_system() -> Result<(), Error> {
        // Declare a map of VirtualKeyCode and the expected MyInputEvent
        let input_choices: Vec<(VirtualKeyCode, MyInputEvent)> = vec![
            (VirtualKeyCode::Up, MyInputEvent::Up),
            (VirtualKeyCode::Down, MyInputEvent::Down),
            (VirtualKeyCode::Left, MyInputEvent::Left),
            (VirtualKeyCode::Right, MyInputEvent::Right),
            (VirtualKeyCode::Space, MyInputEvent::Interact)
        ];

        // First we declare a series of key presses.
        // These are transformed into "Release <key>, Press <key>" messages in the Input system.
        let events: Vec<(VirtualKeyCode, MyInputEvent)> = vec![
            input_choices[0], // Up
            input_choices[2], // Left
            input_choices[2], // Left
            input_choices[1], // Down
            input_choices[1], // Down
            input_choices[1], // Down
            input_choices[3], // Right
            input_choices[3], // Right
            input_choices[3], // Right
            input_choices[3], // Right
            input_choices[4], // Space
            input_choices[3], // Right
            input_choices[3], // Right
        ];

        // We collect just the InputSystem messages to send
        let keys: Vec<VirtualKeyCode> = events.iter().map(|x| x.0).collect();

        // We collet the corresponding MyInputSystem messages we expect to see
        let expected_events: Vec<MyInputEvent> = events.iter().map(|x| x.1).collect();

        // Finally we initialze an Amethyst Test Application.
        let mut app = AmethystApplication::ui_base::<StringBindings>()
            // This application includes our monolithic Bundle.
            // We could just include the MyInputSystem if we wanted to be more targeted.
            .with_bundle(EVG1Bundle);
        
        // Initialize a MyInputEvent Channel Reader
        // Because this reader is initialized before any Input events are generated, it should
        // capture all of the synthetic user interaction we are going to generate with the `events`
        // Vec from above.
        app = app.with_effect(|world: &mut World| {
                let reader = world.system_data::<Write<EventChannel<MyInputEvent>>>().register_reader();
                world.insert(reader);
            });

        // For each of the VirtualKeyCodes we generated above we are going to add a "key release"
        // and then a "key press" event.
        // 
        // We add three `with_effect` blocks because sending all of the events in one `with_effect`
        // block causes none of the signals to be sent.
        //
        // Each `with_*` seems to happen at the same instant, and subsequent `with_*` happen in the
        // following instant -- which is good to know!
        for key in  keys.clone() {
            // Start by making sure the key is released
            app = app.with_effect(move |world| {
                // Press the up key
                let mut handler: Write<InputHandler<StringBindings>> = world.system_data();
                let mut events: Write<EventChannel<InputEvent<StringBindings>>> = world.system_data();

                handler.send_event(&key_release(key), &mut events, HIDPI);
                assert!(!handler.key_is_down(key));
            })
            // Press the key
            .with_effect(move |world| {
                // Press the up key
                let mut handler: Write<InputHandler<StringBindings>> = world.system_data();
                let mut events: Write<EventChannel<InputEvent<StringBindings>>> = world.system_data();

                handler.send_event(&key_press(key), &mut events, HIDPI);
                assert!(handler.key_is_down(key));
            })
            // Add a second key press event.
            //
            // We double press to verify that our MyInputEvent channel only recieves one MyInputEvent per key press
            // This should not cause a second MyInputEvent to be added to the channel.
            .with_effect(move |world| {
                // Press the up key
                let mut handler: Write<InputHandler<StringBindings>> = world.system_data();
                let mut events: Write<EventChannel<InputEvent<StringBindings>>> = world.system_data();

                assert!(handler.key_is_down(key));
                handler.send_event(&key_press(key), &mut events, HIDPI);
                assert!(handler.key_is_down(key));
            })
        }

        // Finally we make assertions about the generated events from above.
        //
        // To review:
        // - Each key press should generate one MyInputEvent in the MyInputEvent EventChannel.
        // - Duplicate key presses should not generate additional messages
        app.with_assertion(|world: &mut World| {
                // The MyInputEvent channel recieved the correct messages
                assert!(world.has_value::<EventChannel<MyInputEvent>>());
                let channel: Read<EventChannel<MyInputEvent>> = world.system_data();

                assert!(world.has_value::<ReaderId<MyInputEvent>>());
                let mut reader = world.fetch_mut::<ReaderId<MyInputEvent>>();

                // VERIFY the "up" key is pressed
                // VERIFY there were messages
                for (actual, expected) in channel.read(&mut reader).zip(expected_events) {
                    assert_eq!(&expected, actual);
                }
            })
            .run()
    }

    // Shamelessly ripped from https://docs.amethyst.rs/stable/src/amethyst_input/input_handler.rs.html
    fn key_press(virtual_keycode: VirtualKeyCode) -> Event {
        key_event(101, virtual_keycode, ElementState::Pressed)
    }

    // Shamelessly ripped from https://docs.amethyst.rs/stable/src/amethyst_input/input_handler.rs.html
    fn key_release(virtual_keycode: VirtualKeyCode) -> Event {
        key_event(101, virtual_keycode, ElementState::Released)
    }

    // Shamelessly ripped from https://docs.amethyst.rs/stable/src/amethyst_input/input_handler.rs.html
    fn key_event(
        scancode: ScanCode,
        virtual_keycode: VirtualKeyCode,
        state: ElementState,
    ) -> Event {
        Event::WindowEvent {
            window_id: unsafe { WindowId::dummy() },
            event: WindowEvent::KeyboardInput {
                device_id: unsafe { DeviceId::dummy() },
                input: KeyboardInput {
                    scancode,
                    state,
                    virtual_keycode: Some(virtual_keycode),
                    modifiers: ModifiersState {
                        shift: false,
                        ctrl: false,
                        alt: false,
                        logo: false,
                    },
                },
            },
        }
    }
}

///
/// Movable Component
///
#[derive(Clone, Copy)]
struct Movable {
    pos: (u8, u8),
}

impl Component for Movable {
    type Storage = DenseVecStorage<Self>;
}

impl Movable {
    fn get_pos(self) -> (u8, u8) {
        self.pos
    }

    fn move_up(&mut self) {
        println!("⬆️  {:?}", self.get_pos());
        self.pos.0 = self.pos.0 + 1;
    }

    fn move_down(&mut self) {
        println!("⬇️  {:?}", self.get_pos());
        match self.pos.0.checked_sub(1) {
            Some(res) => self.pos.0 = res,
            None => (),
        }
    }

    fn move_right(&mut self) {
        println!("➡️  {:?}", self.get_pos());
        self.pos.1 = self.pos.1 + 1;
    }

    fn move_left(&mut self) {
        println!("⬅️  {:?}", self.get_pos());
        match self.pos.1.checked_sub(1) {
            Some(res) => self.pos.1 = res,
            None => (),
        }
    }

    fn interact(self) {
        println!("🎯 {:?}", self.get_pos());
    }

    fn new() -> Self {
        Movable { pos: (0,0) }
    }
}

impl Default for Movable {
    fn default() -> Self { Movable::new() }
}

///
/// Movement System
///
#[derive(SystemDesc)]
#[system_desc(name(MovementSystemDesc))]
struct MovementSystem {
    #[system_desc(event_channel_reader)]
    reader: ReaderId<MyInputEvent>,
}

impl MovementSystem {
    fn new(reader: ReaderId<MyInputEvent>) -> Self {
        MovementSystem { reader }
    }
}

///
/// Reader system for input
///
impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        Read<'a, EventChannel<MyInputEvent>>,
        WriteExpect<'a, Movable>,
        // TODO: The reason we're getting an error here is that no entities exist with both a listener
        // and a movable struct.
        // we need to make each entity a listener.
    );

    ///
    /// ...
    ///
    fn run(&mut self, (input_event_channel, mut movable): Self::SystemData) {
        for event in input_event_channel.read(&mut self.reader) {
            match event {
                MyInputEvent::Up => movable.move_up(),
                MyInputEvent::Down => movable.move_down(),
                MyInputEvent::Right => movable.move_right(),
                MyInputEvent::Left => movable.move_left(),
                MyInputEvent::Interact => movable.interact(),
            };
        }
    }
}

#[derive(Debug)]
struct EVG1Bundle;

impl<'a, 'b> SystemBundle<'a, 'b> for EVG1Bundle {
    fn build(
        self,
        world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        world.insert(EventChannel::<MyInputEvent>::new());
        builder.add(MyInputSystem::new(), "input_transform_system", &[]);
        builder.add(
            MovementSystemDesc::default().build(world),
            "movement_system",
            &["input_transform_system"],
        );
        Ok(())
    }
}

fn main() -> amethyst::Result<()> {
    env::set_var("WINIT_UNIX_BACKEND", "x11");

    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.66, 0.22, 0.22, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(
            TransformBundle::new()
        )?
        .with_bundle(
            InputBundle::<StringBindings>::new()
        )?
        .with_bundle(
            EVG1Bundle
        )?;

    let mut game = Application::new(
        assets_dir,
        GameState,
        game_data,
    )?;

    game.run();

    Ok(())
}
