//! Extension trait for creating transitions on GPUI windows.

use std::time::Duration;

use gpui::{App, Context, ElementId, Window};

use crate::{Lerp, Transition, TransitionState};

/// Extension trait for GPUI's [`Window`] that provides convenient methods for creating transitions.
///
/// This trait adds `use_transition` and `use_keyed_transition` methods to `Window`,
/// allowing you to create animated transitions that integrate with GPUI's state management.
///
/// # Example
///
/// ```ignore
/// use gpui_transitions::WindowUseTransition;
/// use std::time::Duration;
///
/// // In a render function or element
/// let opacity = window.use_keyed_transition(
///     "fade-in",
///     cx,
///     Duration::from_millis(200),
///     |_, _| 0.0_f32,
/// );
///
/// // Trigger the animation
/// opacity.update(cx, |val, cx| {
///     *val = 1.0;
///     cx.notify();
/// });
/// ```
pub trait WindowUseTransition {
    /// Creates a new transition with automatic state management.
    ///
    /// The state for this transition is managed internally and will be recreated
    /// on each render. For persistent state across renders, use [`use_keyed_transition`](Self::use_keyed_transition).
    ///
    /// # Arguments
    ///
    /// * `cx` - The GPUI application context.
    /// * `duration` - How long the transition should take to complete.
    /// * `initial_goal` - A closure that returns the initial value for the transition.
    ///
    /// # Returns
    ///
    /// A [`Transition`] that can be used to animate values.
    fn use_transition<T: Lerp + Clone + PartialEq + 'static>(
        &mut self,
        cx: &mut App,
        duration: Duration,
        initial_goal: impl Fn(&mut Window, &mut Context<TransitionState<T>>) -> T,
    ) -> Transition<T>;

    /// Creates a new keyed transition with persistent state.
    ///
    /// The state for this transition is associated with the provided key and will
    /// persist across renders as long as the key remains the same. This is the
    /// recommended method for most use cases where you want smooth, continuous
    /// animations.
    ///
    /// # Arguments
    ///
    /// * `key` - A unique identifier for this transition's state. Can be a string,
    ///   number, or any type that implements `Into<ElementId>`.
    /// * `cx` - The GPUI application context.
    /// * `duration` - How long the transition should take to complete.
    /// * `initial_goal` - A closure that returns the initial value for the transition.
    ///   This is only called when the state is first created.
    ///
    /// # Returns
    ///
    /// A [`Transition`] that can be used to animate values.
    fn use_keyed_transition<T: Lerp + Clone + PartialEq + 'static>(
        &mut self,
        key: impl Into<ElementId>,
        cx: &mut App,
        duration: Duration,
        initial_goal: impl Fn(&mut Window, &mut Context<TransitionState<T>>) -> T,
    ) -> Transition<T>;
}

impl WindowUseTransition for Window {
    fn use_transition<T: Lerp + Clone + PartialEq + 'static>(
        &mut self,
        cx: &mut App,
        duration: Duration,
        init: impl Fn(&mut Window, &mut Context<TransitionState<T>>) -> T,
    ) -> Transition<T> {
        let state = self.use_state(cx, |window, cx| TransitionState::new(init(window, cx)));

        Transition::new(state, duration)
    }

    fn use_keyed_transition<T: Lerp + Clone + PartialEq + 'static>(
        &mut self,
        key: impl Into<ElementId>,
        cx: &mut App,
        duration: Duration,
        init: impl Fn(&mut Window, &mut Context<TransitionState<T>>) -> T,
    ) -> Transition<T> {
        let state =
            self.use_keyed_state(key, cx, |window, cx| TransitionState::new(init(window, cx)));

        Transition::new(state, duration)
    }
}
