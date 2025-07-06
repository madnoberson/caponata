/// Specifies how the animation repeats over time.
///
/// Default variant is [`AnimationRepeatMode::Infinite`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum AnimationRepeatMode {
    /// The animation repeats a full cycle (all steps)
    /// indefinitely.
    #[default]
    Infinite,

    /// The animation repeats a full cycle (all steps)
    /// a fixed number of times.
    Finite(u16),
}
