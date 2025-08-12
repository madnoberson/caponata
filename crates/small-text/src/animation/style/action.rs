use ratatui::style::{
    Color,
    Modifier,
};

/// A single action in the text animation step.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnimationAction {
    UpdateCharacter(char),
    UpdateForegroundColor(Color),
    UpdateBackgroundColor(Color),
    AddModifier(Modifier),
    RemoveModifier(Modifier),
    RemoveAllModifiers,
}
