/// Type of animation for [`SmallSpinnerWidget`].
///
/// Default variant is [`SmallSpinnerType::BrailleDouble`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum SmallSpinnerType {
    /// ["⠘", "⠰", "⠤", "⠆", "⠃", "⠉"]
    #[default]
    BrailleDouble,
}
