use super::SmallSpinnerType;

/// A struct that cycles through a sequence of symbols used for
/// rendering spinners.
///
/// The cycle is determined by the [`SmallSpinnerType`] provided
/// on initialization. It keeps track of the current symbol and
/// allows advancing to the next one in the sequence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SymbolCycle {
    symbols: Vec<&'static str>,
    max_index: usize,
    current_index: usize,
}

impl Default for SymbolCycle {
    fn default() -> Self {
        Self::new(SmallSpinnerType::default())
    }
}

impl SymbolCycle {
    pub fn new(spinner_type: SmallSpinnerType) -> Self {
        let symbols = match spinner_type {
            SmallSpinnerType::BrailleDouble => {
                vec!["⠘", "⠰", "⠤", "⠆", "⠃", "⠉"]
            }
        };
        let max_index = symbols.clone().len() - 1;

        Self {
            symbols,
            max_index,
            current_index: 0,
        }
    }

    /// Returns the currently selected symbol in the cycle.
    ///
    /// This does not modify the internal state of the cycle.
    pub fn current_symbol(&self) -> &'static str {
        self.symbols[self.current_index]
    }

    /// Advances to the next symbol in the cycle and returns it.
    ///
    /// If the current symbol is the last one, it wraps around
    /// to the first.
    pub fn next_symbol(&mut self) -> &'static str {
        if self.current_index != self.max_index {
            self.current_index += 1;
        } else {
            self.current_index = 0;
        }
        self.symbols[self.current_index]
    }

    /// Resets the cycle to the first symbol.
    pub fn reset(&mut self) {
        self.current_index = 0;
    }
}
