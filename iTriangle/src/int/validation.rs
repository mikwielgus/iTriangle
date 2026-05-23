use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::IntOverlayOptions;
use i_overlay::i_float::int::number::int::IntNumber;

#[derive(Debug, Clone, Copy)]
pub struct Validation<I: IntNumber> {
    pub fill_rule: FillRule,
    pub options: IntOverlayOptions<I::WideUInt>,
}

impl<I: IntNumber> Validation<I> {
    pub fn with_fill_rule(fill_rule: FillRule) -> Self {
        Self {
            fill_rule,
            options: IntOverlayOptions::keep_output_points(),
        }
    }
}

impl<I: IntNumber> Default for Validation<I> {
    fn default() -> Self {
        Self {
            fill_rule: FillRule::NonZero,
            options: IntOverlayOptions::keep_output_points(),
        }
    }
}
