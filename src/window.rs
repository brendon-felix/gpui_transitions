use std::time::Duration;

use gpui::{App, Context, ElementId, Window};

use crate::{Lerp, Transition, TransitionState};

pub trait WindowUseTransition {
    fn use_transition<T: Lerp + Clone + PartialEq + 'static>(
        &mut self,
        cx: &mut App,
        duration: Duration,
        initial_goal: impl Fn(&mut Window, &mut Context<TransitionState<T>>) -> T,
    ) -> Transition<T>;

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
