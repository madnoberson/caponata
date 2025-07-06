/// Represents the selection of symbol positions to
/// which styles should be applied during a specific
/// step of the animation.
///
/// Priority of applying:
///
/// 1. [`AnimationTarget::Single`]
/// 2. [`AnimationTarget::Range`]
/// 3. [`AnimationTarget::Untouched`]
/// 4. [`AnimationTarget::UntouchedThisStep`]
///
/// Default variant is [`AnimationTarget::Untouched`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AnimationTarget {
    /// A specific position of a single symbol. This
    /// is a virtual X coordinate representing the
    /// offset from the beginning of the text.
    Single(u16),

    /// A range of symbol positions (inclusive).
    /// The first value is the start, and the second
    /// is the end of the range. These are virtual
    /// X coordinates representing the offset from
    /// the beginning of the text.
    Range(u16, u16),

    /// Positions of symbols that were not affected
    /// by styling at any step.
    #[default]
    Untouched,

    /// Positions of symbols that were not affected
    /// by styling during the current animation step.
    UntouchedThisStep,
}
