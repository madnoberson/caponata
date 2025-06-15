#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ButtonEvent {
    /// Triggered when a [`ButtonWidget`] is clicked
    /// with the left mouse button.
    Clicked,

    /// Triggered when the mouse cursor enters the area
    /// of a [`ButtonWidget`]. The event includes a
    /// boolean flag indicating whether the widget was
    /// already hovered.
    Hovered(bool),

    /// Triggered when the mouse cursor leaves the area
    /// of a [`ButtonWidget`] that was previously hovered.
    Unhovered,
}
