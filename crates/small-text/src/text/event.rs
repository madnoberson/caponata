use super::Symbol;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SmallTextEvent {
    Hovered(Symbol),
    HoveredSymbolChanged(Symbol),
    Unhovered,
    Pressed(Symbol),
    Released(Symbol),
}
