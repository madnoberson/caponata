#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ButtonStatus {
    #[default]
    Normal,
    Hovered,
    Pressed,
    Disabled,
}
