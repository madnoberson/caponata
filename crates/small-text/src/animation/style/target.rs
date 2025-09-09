use std::{
    cmp::Ordering,
    collections::HashMap,
};

use caponata_common::Callable;

use crate::StepSymbolState;

type AnimationTargetCustomCallable =
    Callable<(HashMap<u16, StepSymbolState>,), Box<dyn Iterator<Item = u16>>>;

/// Represents the selection of symbol positions to
/// which styles should be applied during a specific
/// step of the animation.
///
/// # Applying order:
///
/// 1. [`AnimationTarget::Custom`]
/// 2. [`AnimationTarget::Every`]
/// 3. [`AnimationTarget::EveryFrom`]
/// 4. [`AnimationTarget::ExceptEvery`]
/// 5. [`AnimationTarget::ExceptEveryFrom`]
/// 6. [`AnimationTarget::Range`]
/// 7. [`AnimationTarget::Single`]
/// 8. [`AnimationTarget::Untouched`]
/// 9. [`AnimationTarget::UntouchedThisStep`]
///
/// Default variant is [`AnimationTarget::Untouched`].
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
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

    /// Custom selection logic using a function.
    /// The function receives a hashmap of virtual
    /// x coordinates with corresponding symbol
    /// states and should return the selected
    /// virtual x coordinates.
    Custom(AnimationTargetCustomCallable),

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
        AnimationTarget::Custom(_) => 8,
        AnimationTarget::Every(_) => 7,
        AnimationTarget::EveryFrom(_, _) => 6,
        AnimationTarget::ExceptEvery(_) => 5,
        AnimationTarget::ExceptEveryFrom(_, _) => 4,
        AnimationTarget::Range(_, _) => 3,
        AnimationTarget::Single(_) => 2,
        AnimationTarget::Untouched => 1,
        AnimationTarget::UntouchedThisStep => 0,
    };
    priority(&a).cmp(&priority(&b))
}
