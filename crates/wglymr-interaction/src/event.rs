#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct KeyModifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum MouseEventKind {
    Move,
    Down(MouseButton),
    Up(MouseButton),
    Wheel { delta: f32 },
    Enter(MouseButton),
    Leave(MouseButton),
}

#[derive(Debug, Clone, Copy)]
pub struct MouseEvent {
    pub kind: MouseEventKind,
    pub screen_pos: [f32; 2],
}
