//! A wrapper type for animating boolean-like values with smooth transitions.

use std::ops::Sub;

/// A wrapper type for animating boolean-like values.
///
/// `BoolLerp` allows you to smoothly transition between two states (true/false)
/// by representing them as numeric values (1.0/0.0) with intermediate values
/// during the animation.
///
/// This is useful for animating properties like opacity, visibility, or any
/// other property that logically represents an on/off state but benefits from
/// smooth transitions.
///
/// # Type Parameter
///
/// * `N` - The numeric type used to represent the value. Typically `f32` or `f64`.
///
/// # Example
///
/// ```ignore
/// use gpui_transitions::BoolLerp;
///
/// // Create from a boolean
/// let visible: BoolLerp<f32> = true.into();
/// assert_eq!(visible.value(), 1.0);
///
/// // Create explicitly
/// let hidden = BoolLerp::<f32>::falsey();
/// assert_eq!(hidden.value(), 0.0);
///
/// // Toggle the value
/// let toggled = visible.toggle();
/// assert_eq!(toggled.value(), 0.0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoolLerp<N> {
    value: N,
}

impl<N: From<u8>> BoolLerp<N> {
    /// Creates a `BoolLerp` representing a "true" state (value = 1).
    pub fn truthy() -> Self {
        BoolLerp { value: N::from(1) }
    }

    /// Creates a `BoolLerp` representing a "false" state (value = 0).
    pub fn falsey() -> Self {
        BoolLerp { value: N::from(0) }
    }
}

impl<N> BoolLerp<N>
where
    N: PartialOrd + From<u8>,
{
    pub(crate) fn new(n: N) -> Self {
        debug_assert!(
            n >= N::from(0) && n <= N::from(1),
            "intermediate value must be between 0 and 1"
        );

        BoolLerp { value: n }
    }
}

impl<N: Copy> BoolLerp<N> {
    /// Returns the underlying numeric value.
    ///
    /// During a transition, this will be an intermediate value between 0 and 1.
    /// When the transition is complete, it will be either 0 (false) or 1 (true).
    pub fn value(&self) -> N {
        self.value
    }
}

impl<N: Copy + From<u8> + Sub<Output = N>> BoolLerp<N> {
    /// Returns a new `BoolLerp` with the toggled value.
    ///
    /// This inverts the value by computing `1 - value`. For a fully true value (1.0),
    /// this returns a fully false value (0.0), and vice versa. Intermediate values
    /// are also inverted (e.g., 0.3 becomes 0.7).
    pub fn toggle(&self) -> Self {
        BoolLerp {
            value: N::from(1) - self.value(),
        }
    }
}

impl<N: From<u8>> From<bool> for BoolLerp<N> {
    /// Converts a boolean into a `BoolLerp`.
    ///
    /// `true` becomes a value of 1, and `false` becomes a value of 0.
    fn from(value: bool) -> BoolLerp<N> {
        BoolLerp {
            value: N::from(value as u8),
        }
    }
}

#[cfg(all(test, feature = "test-support"))]
mod tests {
    use super::*;
    use crate::Lerp;

    #[test]
    fn test_truthy_creation() {
        let truthy: BoolLerp<f32> = BoolLerp::truthy();
        assert_eq!(truthy.value(), 1.0);
    }

    #[test]
    fn test_falsey_creation() {
        let falsey: BoolLerp<f32> = BoolLerp::falsey();
        assert_eq!(falsey.value(), 0.0);
    }

    #[test]
    fn test_from_bool_true() {
        let truthy: BoolLerp<f32> = true.into();
        assert_eq!(truthy.value(), 1.0);
    }

    #[test]
    fn test_from_bool_false() {
        let falsey: BoolLerp<f32> = false.into();
        assert_eq!(falsey.value(), 0.0);
    }

    #[test]
    fn test_toggle_from_true() {
        let truthy: BoolLerp<f32> = BoolLerp::truthy();
        let toggled = truthy.toggle();
        assert_eq!(toggled.value(), 0.0);
    }

    #[test]
    fn test_toggle_from_false() {
        let falsey: BoolLerp<f32> = BoolLerp::falsey();
        let toggled = falsey.toggle();
        assert_eq!(toggled.value(), 1.0);
    }

    #[test]
    fn test_double_toggle() {
        let original: BoolLerp<f32> = BoolLerp::truthy();
        let double_toggled = original.toggle().toggle();
        assert_eq!(double_toggled.value(), original.value());
    }

    #[test]
    fn test_bool_lerp_interpolation() {
        let start: BoolLerp<f32> = BoolLerp::falsey();
        let end: BoolLerp<f32> = BoolLerp::truthy();

        let at_start = start.lerp(&end, 0.0);
        assert_eq!(at_start.value(), 0.0);

        let mid = start.lerp(&end, 0.5);
        assert_eq!(mid.value(), 0.5);

        let at_end = start.lerp(&end, 1.0);
        assert_eq!(at_end.value(), 1.0);
    }

    #[test]
    fn test_bool_lerp_reverse_interpolation() {
        let start: BoolLerp<f32> = BoolLerp::truthy();
        let end: BoolLerp<f32> = BoolLerp::falsey();

        let mid = start.lerp(&end, 0.5);
        assert_eq!(mid.value(), 0.5);

        let at_end = start.lerp(&end, 1.0);
        assert_eq!(at_end.value(), 0.0);
    }

    #[test]
    fn test_bool_lerp_partial_values() {
        let start: BoolLerp<f32> = BoolLerp::falsey();
        let end: BoolLerp<f32> = BoolLerp::truthy();

        let quarter = start.lerp(&end, 0.25);
        assert_eq!(quarter.value(), 0.25);

        let three_quarter = start.lerp(&end, 0.75);
        assert_eq!(three_quarter.value(), 0.75);
    }

    #[test]
    fn test_bool_lerp_equality() {
        let a: BoolLerp<f32> = BoolLerp::truthy();
        let b: BoolLerp<f32> = BoolLerp::truthy();
        assert_eq!(a, b);

        let c: BoolLerp<f32> = BoolLerp::falsey();
        assert_ne!(a, c);
    }

    #[test]
    fn test_bool_lerp_clone() {
        let original: BoolLerp<f32> = BoolLerp::truthy();
        let cloned = original.clone();
        assert_eq!(original.value(), cloned.value());
    }

    #[test]
    fn test_bool_lerp_copy() {
        let original: BoolLerp<f32> = BoolLerp::truthy();
        let copied: BoolLerp<f32> = original;
        assert_eq!(original.value(), copied.value());
    }

    #[test]
    fn test_bool_lerp_f64() {
        let truthy: BoolLerp<f64> = BoolLerp::truthy();
        assert_eq!(truthy.value(), 1.0);

        let falsey: BoolLerp<f64> = BoolLerp::falsey();
        assert_eq!(falsey.value(), 0.0);

        let toggled = truthy.toggle();
        assert_eq!(toggled.value(), 0.0);
    }
}
