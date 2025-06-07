use super::SmallSpinnerType;

/// Returns a cyclic iterator over spinner animation symbols
/// for the given spinner type.
///
/// This function creates an infinite iterator that cycles
/// through a sequence of unicode symbols representing a
/// spinner animation.
pub(crate) fn symbol_cycle_by_spinner_type(
    spinner_type: SmallSpinnerType,
) -> impl Iterator<Item = &'static str> {
    match spinner_type {
        SmallSpinnerType::BrailleDouble => {
            ["⠘", "⠰", "⠤", "⠆", "⠃", "⠉"].into_iter().cycle()
        }
    }
}
