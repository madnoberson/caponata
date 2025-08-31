use std::{
    cmp::Ordering,
    fmt,
    hash::{
        Hash,
        Hasher,
    },
    rc::Rc,
};

/// Represents the selection of symbol positions to which
/// styles should be applied to [`SmallTextWidget`].
///
/// # Applying order:
///
/// 1. [`Target::Custom`]
/// 2. [`Target::Range`]
/// 3. [`Target::Single`]
/// 4. [`Target::Untouched`]
///
/// Default variant is [`Target::Untouched`].
#[derive(Default, Clone)]
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

    /// Custom selection logic using a function pointer.
    /// The function receives a boxed iterator over
    /// virtual x coordinates of symbols and returns
    /// a boxed iterator over the selected virtual x
    /// coordinates.
    Custom(
        Rc<
            dyn Fn(
                Box<dyn Iterator<Item = u16>>,
            ) -> Box<dyn Iterator<Item = u16>>,
        >,
    ),

    /// Positions of symbols that were not affected
    /// by styling.
    #[default]
    Untouched,
}

impl fmt::Debug for Target {
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
        }
    }
}

impl PartialEq for Target {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Single(l0), Self::Single(r0)) => l0 == r0,
            (Self::Range(l0, l1), Self::Range(r0, r1)) => l0 == r0 && l1 == r1,
            (Self::Custom(_), Self::Custom(_)) => true,
            (Self::Untouched, Self::Untouched) => true,
            _ => false,
        }
    }
}

impl Eq for Target {}

impl Hash for Target {
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
        }
    }
}

pub(crate) fn targets_sorter(a: Target, b: Target) -> Ordering {
    let priority = |item: &Target| match item {
        Target::Custom(_) => 3,
        Target::Range(_, _) => 2,
        Target::Single(_) => 1,
        Target::Untouched => 0,
    };
    priority(&a).cmp(&priority(&b))
}
