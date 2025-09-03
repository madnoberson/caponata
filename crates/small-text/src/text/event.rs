use super::Symbol;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InteractionEvent {
    Hovered(Symbol),
    HoveredSymbolChanged(Symbol),
    Unhovered,
    Pressed(Symbol),
    Released(Symbol),
}
