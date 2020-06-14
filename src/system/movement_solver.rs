use amethyst::{
    derive::SystemDesc,
    ecs::{Join, Read, Write, ReadStorage, System, SystemData, Entity, Entities},
    shrev::{EventChannel, ReaderId},
};
use crate::component::{Holding, Position, Movable, Name, Named, Immovable};
use crate::lib::TransformedInputEvent;
use std::collections::{VecDeque, HashSet};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub(crate) struct MovementEvent {
    pub(crate) entity: Entity,
    pub(crate) delta: Position,
    pub(crate) from: Position,
    pub(crate) to: Position,
}

///
/// ...
///
#[derive(SystemDesc)]
pub(crate) struct MovementSolverSystem {
    reader: ReaderId<TransformedInputEvent>,
}

impl MovementSolverSystem {
    pub(crate) fn new(reader: ReaderId<TransformedInputEvent>) -> Self {
        MovementSolverSystem { reader: reader }
    }
}

fn core_movement_event(
    name: &Name,
    delta: &Position,
    movables: &ReadStorage<Movable>,
    positions: &ReadStorage<Position>,
    names: &ReadStorage<Named>,
    holdings: &ReadStorage<Holding>,
    entities: &Entities,
) -> Option<(MovementEvent, bool)> {
    for (_m, p, n, h, e) in (movables, positions, names, holdings, entities).join() {
        // I'm not sure how to only grab the entity where this is true...
        if &n.get() == name {
            // Create the movement object
            let event = MovementEvent { entity: e, delta: *delta, from: *p, to: *p + *delta };

            // Capture if the entity is moving
            let is_holding = h.status();

            return Some((event, is_holding))
        }
    }
    return None
}

fn add_all_holding(
    delta: &Position,
    movables: &ReadStorage<Movable>,
    positions: &ReadStorage<Position>,
    holdings: &ReadStorage<Holding>,
    entities: &Entities,
) -> VecDeque<MovementEvent> {
    let mut q: VecDeque<MovementEvent> = VecDeque::new();

    for (_m, p, h, e) in (movables, positions, holdings, entities).join() {
        if h.status() {
            let event = MovementEvent { entity: e, delta: *delta, from: *p , to: *p + *delta };
            q.push_back(event);
        }
    }

    return q
}


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

        for (_m, p, e) in (movables, positions, entities).join() {
            // The event we will add, if the stars align 
            let event = MovementEvent { entity: e, delta: *delta, from: *p, to: *p + *delta };

            // If the movement set doesn't contain this
            // And these two entities would collide
            if would_collide(&move_event.to, p) && !move_set.contains(&event) {
                // Add it to the set
                move_set.insert(event);
                move_queue.push_front(event);
            }
        }
    }

    return move_set
}

fn would_hit_obstacle(
    move_set: &HashSet<MovementEvent>,
    immovables: &ReadStorage<Immovable>,
    positions: &ReadStorage<Position>,
) -> bool {
    for move_action in move_set {
        for (_immovable, position) in (immovables, positions).join() {
            if would_collide(&move_action.to, position) {
                return true
            }
        }
    }
    return false
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

    fn run(&mut self, (input_channel, mut move_channel, movables, immovables, positions, names, holdings, entities): Self::SystemData) {
        // This code is so complicated I throw up a little every time I see it...
        for event in input_channel.read(&mut self.reader) {
            let delta = match event {
                TransformedInputEvent::Up       => Position::new(0, 1),
                TransformedInputEvent::Down     => Position::new(0, -1),
                TransformedInputEvent::Left     => Position::new(-1, 0),
                TransformedInputEvent::Right    => Position::new(1, 0),
                TransformedInputEvent::Interact => Position::new(0,0),
            };

            let name = match event {
                TransformedInputEvent::Up       | TransformedInputEvent::Down  => Name::Vertical,
                TransformedInputEvent::Left     | TransformedInputEvent::Right => Name::Horizontal,
                TransformedInputEvent::Interact                                => Name::Interact,
            };

            if let Some((core, is_holding)) = core_movement_event(&name, &delta, &movables, &positions, &names, &holdings, &entities) {
                let mut move_queue: VecDeque<MovementEvent> = if is_holding {
                    add_all_holding(&delta, &movables, &positions, &holdings, &entities)
                } else {
                    VecDeque::new()
                };
                move_queue.push_front(core);

                let move_set: HashSet<MovementEvent> = add_all_pushed(&mut move_queue, &delta, &movables, &positions, &entities);

                // Verify none of the entities will collide with a wall...
                if !would_hit_obstacle(&move_set, &immovables, &positions) {
                    move_channel.iter_write(move_set);
                } // else do nothing
            }
        }
    }
}

fn would_collide(Position { x: x1, y: y1 }: &Position, Position { x: x2, y: y2 }: &Position) -> bool {
    x1 == x2 && y1 == y2
}
