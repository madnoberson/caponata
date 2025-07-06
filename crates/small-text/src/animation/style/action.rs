use ratatui::style::{
    Color,
    Modifier,
};

/// A single action in the text animation step.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AnimationAction {
    UpdateForegroundColor(Color),
    UpdateBackgroundColor(Color),
    AddModifier(Modifier),
    RemoveModifier(Modifier),
    RemoveAllModifiers,
}
