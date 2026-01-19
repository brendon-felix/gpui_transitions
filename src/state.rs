//! Internal state management for transitions.

use std::time::Instant;

use crate::Lerp;

/// Internal state container for a [`Transition`](crate::Transition).
///
/// This struct holds the data necessary to track a transition's progress,
/// including the start and end goals, timing information, and the last
/// computed delta value.
///
/// You typically don't need to interact with this type directly. It's created
/// and managed by [`Transition`](crate::Transition) and the
/// [`WindowUseTransition`](crate::WindowUseTransition) methods.
///
/// # Type Parameter
///
/// * `T` - The type of value being transitioned. Must implement [`Lerp`], [`Clone`],
///   and [`PartialEq`].
#[derive(Clone)]
pub struct TransitionState<T: Lerp + Clone + PartialEq + 'static> {
    pub(crate) goal_last_updated_at: Option<Instant>,
    pub(crate) initial_goal: T,
    pub(crate) start_goal: T,
    pub(crate) end_goal: T,
    pub(crate) last_delta: f32,
}

impl<T: Lerp + Clone + PartialEq + 'static> TransitionState<T> {
    /// Creates a new transition state with the given initial goal.
    ///
    /// The start goal, end goal, and initial goal are all set to the provided value.
    /// The transition begins in a "completed" state (delta = 1.0) until the goal
    /// is updated.
    pub fn new(initial_goal: T) -> Self {
        Self {
            goal_last_updated_at: None,
            initial_goal: initial_goal.clone(),
            start_goal: initial_goal.clone(),
            end_goal: initial_goal,
            last_delta: 1.,
        }
    }
}

#[cfg(all(test, feature = "test-support"))]
mod tests {
    use super::*;
    use crate::BoolLerp;
    use gpui::{Point, Rgba};

    #[test]
    fn test_new_state_initialization() {
        let state = TransitionState::new(42.0_f32);

        assert_eq!(state.initial_goal, 42.0);
        assert_eq!(state.start_goal, 42.0);
        assert_eq!(state.end_goal, 42.0);
        assert_eq!(state.last_delta, 1.0);
        assert!(state.goal_last_updated_at.is_none());
    }

    #[test]
    fn test_state_with_point() {
        let initial: Point<f32> = Point { x: 10.0, y: 20.0 };
        let state = TransitionState::new(initial.clone());

        assert_eq!(state.initial_goal.x, 10.0);
        assert_eq!(state.initial_goal.y, 20.0);
        assert_eq!(state.start_goal.x, 10.0);
        assert_eq!(state.end_goal.x, 10.0);
    }

    #[test]
    fn test_state_with_rgba() {
        let initial = Rgba {
            r: 1.0,
            g: 0.5,
            b: 0.0,
            a: 1.0,
        };
        let state = TransitionState::new(initial);

        assert_eq!(state.initial_goal.r, 1.0);
        assert_eq!(state.initial_goal.g, 0.5);
        assert_eq!(state.initial_goal.b, 0.0);
        assert_eq!(state.initial_goal.a, 1.0);
    }

    #[test]
    fn test_state_with_bool_lerp() {
        let initial: BoolLerp<f32> = BoolLerp::truthy();
        let state = TransitionState::new(initial);

        assert_eq!(state.initial_goal.value(), 1.0);
        assert_eq!(state.end_goal.value(), 1.0);
    }

    #[test]
    fn test_state_clone() {
        let state = TransitionState::new(100.0_f32);
        let cloned = state.clone();

        assert_eq!(state.initial_goal, cloned.initial_goal);
        assert_eq!(state.start_goal, cloned.start_goal);
        assert_eq!(state.end_goal, cloned.end_goal);
        assert_eq!(state.last_delta, cloned.last_delta);
    }

    #[test]
    fn test_state_with_integer() {
        let state = TransitionState::new(50_i32);

        assert_eq!(state.initial_goal, 50);
        assert_eq!(state.start_goal, 50);
        assert_eq!(state.end_goal, 50);
    }

    #[test]
    fn test_state_starts_completed() {
        let state = TransitionState::new(0.0_f32);

        // last_delta should be 1.0 indicating the transition is "complete"
        assert_eq!(state.last_delta, 1.0);
    }
}
