use std::cmp::Ordering;

use ratatui_common::Callback;

type TargetCustomCallback =
    Callback<(Box<dyn Iterator<Item = u16>>,), Box<dyn Iterator<Item = u16>>>;

/// Represents the selection of symbol positions to which
/// styles should be applied to [`SmallTextWidget`].
///
/// # Applying order:
///
/// 1. [`Target::Custom`]
/// 2. [`Target::Every`]
/// 3. [`Target::EveryFrom`]
/// 4. [`Target::ExceptEvery`]
/// 5. [`Target::ExceptEveryFrom`]
/// 6. [`Target::Range`]
/// 7. [`Target::Single`]
/// 8. [`Target::Untouched`]
///
/// Default variant is [`Target::Untouched`].
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
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

    /// Every n-th symbol position, starting from
    /// starting position. The first value represents
    /// the interval between selected positions, the
    /// second represents the starting position.
    EveryFrom(u16, u16),

    /// All symbol positions except every n-th one,
    /// starting from 0. The value represents the
    /// interval to skip.
    ExceptEvery(u16),

    /// All symbol positions except every n-th one,
    /// starting from starting positions. The first
    /// value represents the interval to skip, the
    /// second represents the starting position.
    ExceptEveryFrom(u16, u16),

    Custom(TargetCustomCallback),

    /// Positions of symbols that were not affected
    /// by styling.
    #[default]
    Untouched,
}

pub(crate) fn target_sorter(a: Target, b: Target) -> Ordering {
    let priority = |item: &Target| match item {
        Target::Custom(_) => 7,
        Target::Every(_) => 6,
        Target::EveryFrom(_, _) => 5,
        Target::ExceptEvery(_) => 4,
        Target::ExceptEveryFrom(_, _) => 3,
        Target::Range(_, _) => 2,
        Target::Single(_) => 1,
        Target::Untouched => 0,
    };
    priority(&a).cmp(&priority(&b))
}
