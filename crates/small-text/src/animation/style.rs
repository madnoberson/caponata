use std::{
    collections::HashMap,
    time::Duration,
};

use derive_builder::Builder;

use crate::SymbolStyle;

/// Represents the selection of symbol positions to
/// which styles should be applied during a specific
/// step of the animation.
///
/// Priority of applying:
///
/// 1. [`AnimationTargetedSymbols::Single`]
/// 2. [`AnimationTargetedSymbols::Range`]
/// 3. [`AnimationTargetedSymbols::Untouched`]
/// 4. [`AnimationTargetedSymbols::UntouchedThisStep`]
///
/// Default variant is [`AnimationTargetedSymbols::Untouched`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AnimationTargetedSymbols {
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

/// A single step in the text animation.
///
/// # Example
///
/// ```rust
/// use std::{
///    collections::HashMap,
///    time::Duration,
/// };
///
/// use ratatui_small_text::{
///     AnimationTargetedSymbols,
///     AnimationStepBuilder,
///     SymbolStyle,
/// };
///
/// let symbol_styles = HashMap::from([
///     (AnimationTargetedSymbols::Single(0), SymbolStyle::default()),
/// ]);
/// let animation_step = AnimationStepBuilder::default()
///     .with_symbol_styles(symbol_styles)
///     .with_duration(Duration::from_millis(50))
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq, Builder)]
#[builder(setter(prefix = "with", into))]
pub struct AnimationStep {
    /// A map from the selections of the symbol positions
    /// to their corresponding styles.
    #[builder(default)]
    pub(crate) symbol_styles: HashMap<AnimationTargetedSymbols, SymbolStyle>,

    /// The duration of this animation step. Once this
    /// time elapses, the animation advances to the next
    /// step.
    #[builder(default)]
    pub(crate) duration: Duration,
}

/// Specifies how the animation repeats over time.
///
/// Default variant is [`AnimationRepeatMode::Infinite`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum AnimationRepeatMode {
    /// The animation repeats a full cycle (all steps)
    /// indefinitely.
    #[default]
    Infinite,

    /// The animation repeats a full cycle (all steps)
    /// a fixed number of times.
    Finite(u16),
}

/// Specifies how the animation advances. This enum
/// controls whether the animation step advances
/// automatically or must be triggered manually.
///
/// Default variant is [`AnimationAdvanceMode::Auto`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum AnimationAdvanceMode {
    /// The animation advances automatically on every
    /// [`SmallTextWidget::render`] method call, if
    /// the current step has lasted long enough.
    #[default]
    Auto,

    /// The animation advances on every
    /// [`SmallTextWidget::render`] method call only
    /// if [`SmallTextWidget::advance_animation`]
    /// method was called beforehand and the current
    /// step has lasted long enough.
    Manual,
}

/// A styling configuration for the animation.
///
/// # Example
///
/// ```rust
/// use std::{
///    collections::HashMap,
///    time::Duration,
/// };
///
/// use ratatui_small_text::{
///     AnimationTargetedSymbols,
///     SymbolStyle,
///     AnimationRepeatMode,
///     AnimationAdvanceMode,
///     AnimationStepBuilder,
///     AnimationStyleBuilder,
/// };
///
/// let first_step_symbol_styles = HashMap::from([
///     (AnimationTargetedSymbols::Single(0), SymbolStyle::default()),
/// ]);
/// let first_step = AnimationStepBuilder::default()
///     .with_symbol_styles(first_step_symbol_styles)
///     .with_duration(Duration::from_millis(50))
///     .build()
///     .unwrap();
/// let second_step_symbol_styles = HashMap::from([
///     (AnimationTargetedSymbols::Untouched, SymbolStyle::default()),
/// ]);
/// let second_step = AnimationStepBuilder::default()
///     .with_symbol_styles(second_step_symbol_styles)
///     .with_duration(Duration::from_millis(50))
///     .build()
///     .unwrap();
/// let animation_style = AnimationStyleBuilder::default()
///     .with_repeat_mode(AnimationRepeatMode::Infinite)
///     .with_advance_mode(AnimationAdvanceMode::Auto)
///     .with_steps(vec![first_step, second_step])
///     .build()
///     .unwrap();
#[derive(Debug, Default, Clone, PartialEq, Eq, Builder)]
#[builder(setter(prefix = "with", into))]
pub struct AnimationStyle {
    #[builder(default)]
    pub(crate) repeat_mode: AnimationRepeatMode,

    #[builder(default)]
    pub(crate) advance_mode: AnimationAdvanceMode,

    #[builder(default)]
    pub(crate) steps: Vec<AnimationStep>,
}
