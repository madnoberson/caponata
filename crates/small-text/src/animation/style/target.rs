use std::{
    cmp::Ordering,
    collections::HashMap,
    fmt,
    hash::{
        Hash,
        Hasher,
    },
    rc::Rc,
};

use crate::StepSymbolState;

/// Represents the selection of symbol positions to
/// which styles should be applied during a specific
/// step of the animation.
///
/// # Warning
///
/// [`AnimationTarget::Custom`] uses function pointer
/// comparisons which are marked as unpredictable. Two
/// functions with identical behavior and implementation
/// may not compare as equal.
///
/// # Applying order:
///
/// 1. [`AnimationTarget::Custom`]
/// 2. [`AnimationTarget::Range`]
/// 3. [`AnimationTarget::Single`]
/// 4. [`AnimationTarget::Untouched`]
/// 5. [`AnimationTarget::UntouchedThisStep`]
///
/// Default variant is [`AnimationTarget::Untouched`].
#[derive(Default, Clone)]
#[allow(unpredictable_function_pointer_comparisons)]
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

    /// Custom selection logic using a function pointer.
    /// The function receives a reference to a hash map
    /// containing virtual x coordinates of symbols and
    /// their corresponding step symbol states, and
    /// returns a vector of the selected virtual x
    /// coordinates.
    Custom(Rc<dyn Fn(&HashMap<u16, StepSymbolState>) -> Vec<u16>>),

    /// Positions of symbols that were not affected
    /// by styling at any step.
    #[default]
    Untouched,

    /// Positions of symbols that were not affected
    /// by styling during the current animation step.
    UntouchedThisStep,
}

impl fmt::Debug for AnimationTarget {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Single(x) => {
                formatter.debug_tuple("Single").field(x).finish()
            }
            Self::Range(start, end) => formatter
                .debug_tuple("Range")
                .field(start)
                .field(end)
                .finish(),
            Self::Custom(_) => formatter
                .debug_tuple("Custom")
                .field(&"<function>")
                .finish(),
            Self::Untouched => write!(formatter, "Untouched"),
            Self::UntouchedThisStep => write!(formatter, "UntouchedThisStep"),
        }
    }
}

impl PartialEq for AnimationTarget {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Single(l0), Self::Single(r0)) => l0 == r0,
            (Self::Range(l0, l1), Self::Range(r0, r1)) => l0 == r0 && l1 == r1,
            (Self::Custom(_), Self::Custom(_)) => true,
            (Self::Untouched, Self::Untouched) => true,
            (Self::UntouchedThisStep, Self::UntouchedThisStep) => true,
            _ => false,
        }
    }
}

impl Eq for AnimationTarget {}

impl Hash for AnimationTarget {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);

        match self {
            Self::Single(arg0) => arg0.hash(state),
            Self::Range(arg0, arg1) => {
                arg0.hash(state);
                arg1.hash(state);
            }
            Self::Custom(_) => "custom".hash(state),
            Self::Untouched => "untouched".hash(state),
            Self::UntouchedThisStep => "untouched_this_step".hash(state),
        }
    }
}

pub(crate) fn animation_targets_sorter(
    a: AnimationTarget,
    b: AnimationTarget,
) -> Ordering {
    let priority = |item: &AnimationTarget| match item {
        AnimationTarget::Custom(_) => 4,
        AnimationTarget::Single(_) => 3,
        AnimationTarget::Range(_, _) => 2,
        AnimationTarget::Untouched => 1,
        AnimationTarget::UntouchedThisStep => 0,
    };
    priority(&a).cmp(&priority(&b))
}
