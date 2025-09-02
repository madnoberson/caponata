use std::cmp::Ordering;

/// Represents the selection of symbol positions to
/// which styles should be applied during a specific
/// step of the animation.
///
/// # Applying order:
///
/// 1. [`AnimationTarget::Every`]
/// 2. [`AnimationTarget::AllExceptEvery`]
/// 3. [`AnimationTarget::Range`]
/// 4. [`AnimationTarget::Single`]
/// 5. [`AnimationTarget::Untouched`]
/// 6. [`AnimationTarget::UntouchedThisStep`]
///
/// Default variant is [`AnimationTarget::Untouched`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnimationTarget {
    /// A specific position of a single symbol. This
    /// is a virtual x coordinate representing the
    /// offset from the beginning of the text.
    Single(u16),

    /// A range of symbol positions (inclusive).
    /// The first value is the start, and the second
    /// is the end of the range. These are virtual
    /// x coordinates representing the offset from
    /// the beginning of the text.
    Range(u16, u16),

    /// Every n-th symbol position, starting from 0.
    /// The value represents the interval between
    /// selected positions.
    Every(u16),

    /// All symbol positions except every n-th one,
    /// starting from 0. The value represents the
    /// interval to skip.
    AllExceptEvery(u16),

    /// Positions of symbols that were not affected
    /// by styling at any step.
    #[default]
    Untouched,

    /// Positions of symbols that were not affected
    /// by styling during the current animation step.
    UntouchedThisStep,
}

pub(crate) fn animation_target_sorter(
    a: AnimationTarget,
    b: AnimationTarget,
) -> Ordering {
    let priority = |item: &AnimationTarget| match item {
        AnimationTarget::Every(_) => 5,
        AnimationTarget::AllExceptEvery(_) => 4,
        AnimationTarget::Single(_) => 3,
        AnimationTarget::Range(_, _) => 2,
        AnimationTarget::Untouched => 1,
        AnimationTarget::UntouchedThisStep => 0,
    };
    priority(&a).cmp(&priority(&b))
}
