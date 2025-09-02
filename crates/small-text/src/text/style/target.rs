use std::cmp::Ordering;

/// Represents the selection of symbol positions to which
/// styles should be applied to [`SmallTextWidget`].
///
/// # Applying order:
///
/// 1. [`Target::Every`]
/// 2. [`Target::AllExceptEvery`]
/// 3. [`Target::Range`]
/// 4. [`Target::Single`]
/// 5. [`Target::Untouched`]
///
/// Default variant is [`Target::Untouched`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Target {
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
    /// by styling.
    #[default]
    Untouched,
}

pub(crate) fn targets_sorter(a: Target, b: Target) -> Ordering {
    let priority = |item: &Target| match item {
        Target::Every(_) => 4,
        Target::AllExceptEvery(_) => 3,
        Target::Range(_, _) => 2,
        Target::Single(_) => 1,
        Target::Untouched => 0,
    };
    priority(&a).cmp(&priority(&b))
}
