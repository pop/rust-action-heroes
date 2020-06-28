//!
//! # Movement was so complicated it has two systems
//!
//! The only thing here is the MovementSolverSystem struct, so go read about that!
//!

use crate::{
    component::{Holding, Immovable, Movable, Name, Named, Position},
    lib::TransformedInputEvent,
};
use amethyst::{
    derive::SystemDesc,
    ecs::{Entities, Entity, Join, Read, ReadStorage, System, SystemData, Write},
    shrev::{EventChannel, ReaderId},
};
use std::collections::{HashSet, VecDeque};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub(crate) struct MovementEvent {
    pub(crate) entity: Entity,
    pub(crate) delta: Position,
    pub(crate) from: Position,
    pub(crate) to: Position,
}

///
/// It reads in TransformedInputEvents and outputs MovementEvents. Sounds simple enough...
/// This is easily the most complicated system in the game.
///
/// It was initially much more verbose, as a giant `match` with copy-paste almost identical code
/// for every TransformedInputEvent.
/// That wasn't maintainable and I ended up refactoring how movement worked from
/// "Movable.move_up()", etc to generating MovementEvents which the current position and a
/// delta.
///
/// The system I ended up deciding on was one where we do a bunch of logic to figure out which
/// entities _would_ move, then send those movements as events on a channel.
///
/// The nitty gritty of figuring out _which_ entities to move is more complicated than it should
/// be, but since this system doesn't actually _do_ the moving it makes this system a bit more
/// concise than it would be nievely.
///
#[derive(SystemDesc)]
pub(crate) struct MovementSolverSystem {
    reader: ReaderId<TransformedInputEvent>,
}

impl MovementSolverSystem {
    pub(crate) fn new(reader: ReaderId<TransformedInputEvent>) -> Self {
        MovementSolverSystem { reader }
    }
}

impl<'s> System<'s> for MovementSolverSystem {
    type SystemData = (
        Read<'s, EventChannel<TransformedInputEvent>>,
        Write<'s, EventChannel<MovementEvent>>,
        ReadStorage<'s, Movable>,
        ReadStorage<'s, Immovable>,
        ReadStorage<'s, Position>,
        ReadStorage<'s, Named>,
        ReadStorage<'s, Holding>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        (
            input_channel,
            mut move_channel,
            movables,
            immovables,
            positions,
            names,
            holdings,
            entities,
        ): Self::SystemData,
    ) {
        // This code is so complicated I get dizzy every time I see it...
        for event in input_channel.read(&mut self.reader) {
            // Find a delta for the corresponding movement
            let delta = match event {
                TransformedInputEvent::Up => Position::new(0, 1),
                TransformedInputEvent::Down => Position::new(0, -1),
                TransformedInputEvent::Left => Position::new(-1, 0),
                TransformedInputEvent::Right => Position::new(1, 0),
                TransformedInputEvent::Interact => Position::new(0, 0),
            };

            // Find the name of the entity we are starting with
            let name = match event {
                TransformedInputEvent::Up | TransformedInputEvent::Down => Name::Vertical,
                TransformedInputEvent::Left | TransformedInputEvent::Right => Name::Horizontal,
                TransformedInputEvent::Interact => Name::Interact,
            };

            // Get the main entity we are moving and weather it is holding.
            // Use that as a starting place to determine _all_ entities that should move.
            if let Some((core, is_holding)) = core_movement_event(
                &name, &delta, &movables, &positions, &names, &holdings, &entities,
            ) {
                // Get a queue of all entities the `core` entity is holding.
                let mut move_queue: VecDeque<MovementEvent> = if is_holding {
                    add_all_holding(&delta, &movables, &positions, &holdings, &entities)
                } else {
                    VecDeque::new()
                };
                move_queue.push_front(core);

                // Generate a set of all characters we're pushing
                let move_set: HashSet<MovementEvent> =
                    add_all_pushed(&mut move_queue, &delta, &movables, &positions, &entities);

                // Verify none of the entities will collide with a wall...
                // If we would hit a wall, cancel the movement.
                // Not exactly a transaction, but kind of a transaction.
                if !would_hit_obstacle(&move_set, &immovables, &positions) {
                    move_channel.iter_write(move_set);
                } // else do nothing
            }
        }
    }
}

///
/// Generates a MovementEvent for the entity we're moving.
/// Moving Left/Right produces the entity for "Prince horizontival the first" (veritcal), Up/Down ->
/// "Duke vert the pure" (horizontal).
///
/// Once we have that entity, we create a MovementEvent for that characters using the delta given.
///
/// We also return weather that character is in a holding state.
///
fn core_movement_event(
    name: &Name,
    delta: &Position,
    movables: &ReadStorage<Movable>,
    positions: &ReadStorage<Position>,
    names: &ReadStorage<Named>,
    holdings: &ReadStorage<Holding>,
    entities: &Entities,
) -> Option<(MovementEvent, bool)> {
    for (_movable, position, the_name, holding, entity) in
        (movables, positions, names, holdings, entities).join()
    {
        // I'm not sure how to only grab the entity where this is true...
        if &the_name.get() == name {
            // Create the movement object
            let event = MovementEvent {
                entity: entity,
                delta: *delta,
                from: *position,
                to: *position + *delta,
            };

            // Capture if the entity is moving
            let is_holding = holding.status();

            return Some((event, is_holding));
        }
    }
    return None;
}

///
/// Geneartes a queue of all movaable entities in the "holding" state.
///
fn add_all_holding(
    delta: &Position,
    movables: &ReadStorage<Movable>,
    positions: &ReadStorage<Position>,
    holdings: &ReadStorage<Holding>,
    entities: &Entities,
) -> VecDeque<MovementEvent> {
    let mut queue: VecDeque<MovementEvent> = VecDeque::new();

    for (_movable, position, holding, entity) in (movables, positions, holdings, entities).join() {
        if holding.status() {
            let event = MovementEvent {
                entity: entity,
                delta: *delta,
                from: *position,
                to: *position + *delta,
            };
            queue.push_back(event);
        }
    }

    queue
}

///
/// Given a queue of movement events, determines which entities will be pushed.
/// Think Crates, Keys, other characters, etc.
///
fn add_all_pushed(
    move_queue: &mut VecDeque<MovementEvent>,
    delta: &Position,
    movables: &ReadStorage<Movable>,
    positions: &ReadStorage<Position>,
    entities: &Entities,
) -> HashSet<MovementEvent> {
    let mut move_set: HashSet<MovementEvent> = HashSet::new();

    // Pop the next item off the stack
    while let Some(move_event) = move_queue.pop_back() {
        move_set.insert(move_event);

        for (_movable, position, entity) in (movables, positions, entities).join() {
            // The event we will add, if the stars align
            let event = MovementEvent {
                entity: entity,
                delta: *delta,
                from: *position,
                to: *position + *delta,
            };

            // If the movement set doesn't contain this
            // And these two entities would collide
            if would_collide(&move_event.to, position) && !move_set.contains(&event) {
                // Add it to the set
                move_set.insert(event);
                move_queue.push_front(event);
            }
        }
    }

    return move_set;
}

///
/// Given a set of MovementEvents, determines if that set would collide with an immovable object --
/// like a wall.
///
fn would_hit_obstacle(
    move_set: &HashSet<MovementEvent>,
    immovables: &ReadStorage<Immovable>,
    positions: &ReadStorage<Position>,
) -> bool {
    for move_action in move_set {
        for (_immovable, position) in (immovables, positions).join() {
            if would_collide(&move_action.to, position) {
                return true;
            }
        }
    }
    return false;
}

///
/// Very simple method to determine if two positions overlap.
///
/// Based on the name it should probably be a Position and a MovementEvent -- but I'm not
/// demonstrating good API design!
///
fn would_collide(
    Position { x: x1, y: y1 }: &Position,
    Position { x: x2, y: y2 }: &Position,
) -> bool {
    x1 == x2 && y1 == y2
}
