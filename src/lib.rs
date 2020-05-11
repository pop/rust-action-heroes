#[derive(Debug, Copy, Clone)]
pub(crate) enum TransformedInputEvent {
    Up,
    Down,
    Left,
    Right,
    Interact,
}
