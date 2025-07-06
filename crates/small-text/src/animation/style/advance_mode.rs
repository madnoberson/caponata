/// Specifies how the animation advances. This enum
/// controls whether the animation step advances
/// automatically or must be triggered manually.
///
/// Default variant is [`AnimationAdvanceMode::Auto`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum AnimationAdvanceMode {
    /// The animation advances automatically on every
    /// [`SmallTextWidget::render`] method call, if
    /// the current step has lasted long enough.
    #[default]
    Auto,

    /// The animation advances on every
    /// [`SmallTextWidget::render`] method call only
    /// if [`SmallTextWidget::advance_animation`]
    /// method was called beforehand and the current
    /// step has lasted long enough.
    Manual,
}
