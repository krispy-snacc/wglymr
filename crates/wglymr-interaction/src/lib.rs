// WGLYMR Interaction System
// Operators and modal interaction logic.

// Input abstraction
pub mod dispatch;
pub mod event;

pub use dispatch::InputDispatcher;
pub use event::{KeyModifiers, MouseButton, MouseEvent, MouseEventKind};

// Operators
pub mod operator;
pub mod operator_stack;
pub mod operators;

pub use operator::{EditorOperator, OperatorContext, OperatorResult};
pub use operator_stack::OperatorStack;

// Interaction mapping
pub mod map;
pub mod resolver;
pub mod target;

pub use map::map_draw_item_to_target;
pub use resolver::{InteractionResolver, InteractionState};
pub use target::{InteractionTarget, OverlayKind};
