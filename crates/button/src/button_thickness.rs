/// Type of blocks that is added to top and bottom side of
/// a [`ButtonWidget`].
///
/// Default variant is [`ButtonThickness::None`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ButtonThickness {
    /// Add '▔' block to the bottom and '▁' block to
    /// the top.
    #[default]
    OneEightBlock,

    /// Add '▀' block to the bottom and '▄' block to
    /// the top.
    HalfBlock,
}
