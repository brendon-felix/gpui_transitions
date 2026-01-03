use std::time::Duration;

use gpui::{
    AnyElement, App, AppContext, Application, Bounds, Context, Div, ElementId, Interactivity, Menu,
    StyleRefinement, TitlebarOptions, Window, WindowBounds, WindowOptions, div, ease_in_out,
    ease_out_quint, point, prelude::*, px, rgb, size,
};
use gpui_transitions::WindowUseTransition;
use smallvec::SmallVec;

use crate::element_id_ext::ElementIdExt;

#[derive(IntoElement)]
struct Button {
    id: ElementId,
    base: Div,
    children: SmallVec<[AnyElement; 2]>,
}

impl Button {
    fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            base: div(),
            children: SmallVec::new(),
        }
    }
}

impl ParentElement for Button {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for Button {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl InteractiveElement for Button {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl StatefulInteractiveElement for Button {}

impl RenderOnce for Button {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        const HOVER_STRENGTH: f32 = 0.3;

        let hover_transition = window
            .use_keyed_transition(
                self.id.with_suffix("hover"),
                cx,
                Duration::from_millis(200),
                |_window, _cx| 0.,
            )
            .with_easing(ease_in_out);

        let hover_amount = *hover_transition.evaluate(window, cx) * HOVER_STRENGTH;
        let bg_color = gpui::hsla(0., 0., 1. - hover_amount, 1.);

        self.base
            .id(self.id)
            .cursor_pointer()
            .rounded(px(100.))
            .pl(px(14.))
            .pr(px(14.))
            .pt(px(10.))
            .pb(px(10.))
            .bg(bg_color)
            .text_color(rgb(0x110F15))
            .children(self.children)
            .on_hover(move |hover, _window, cx| {
                hover_transition.update(cx, |this, cx| {
                    *this = *hover as u8 as f32;
                    cx.notify();
                });
            })
    }
}

struct Root;

impl Render for Root {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let left_transition = window
            .use_keyed_transition(
                "left-position",
                cx,
                Duration::from_millis(500),
                |_window, _cx| point(px(0.), px(0.)),
            )
            .continuous(true)
            .with_easing(ease_out_quint());

        let right_transition = window
            .use_keyed_transition(
                "right-position",
                cx,
                Duration::from_millis(800),
                |_window, _cx| point(px(0.), px(0.)),
            )
            .continuous(false)
            .with_easing(|t| 1. - ease_out_bounce(t));

        let left_btn_pos = *left_transition.evaluate(window, cx);
        let right_btn_pos = *right_transition.evaluate(window, cx);

        let border_color = gpui::rgb(0x444054);

        div()
            .size_full()
            .flex()
            .justify_center()
            .items_center()
            .absolute()
            .bg(rgb(0x110F15))
            .gap(px(20.))
            .p(px(50.))
            // .child(Button::new("btn-0", true).child("Click me!"))
            .child(
                div().flex().flex_col().gap(px(10.)).children([
                    div()
                        .text_center()
                        .text_color(rgb(0xE6E1E5))
                        .text_xl()
                        .child("Continuous"),
                    div()
                        .flex()
                        .justify_center()
                        .items_center()
                        .size_112()
                        .border_1()
                        .border_color(border_color)
                        .child(
                            Button::new("btn-0")
                                .top(left_btn_pos.y)
                                .left(left_btn_pos.x)
                                .child("Click me!")
                                .on_click(move |_event, _window, cx| {
                                    left_transition.update(cx, |this, cx| {
                                        let tau = std::f32::consts::TAU;
                                        let angle = tau * rand::random::<f32>();
                                        let radius = px(100.);
                                        let new_position =
                                            point(angle.cos() * radius, angle.sin() * radius);
                                        *this = new_position;
                                        cx.notify();
                                    });
                                }),
                        ),
                ]),
            )
            .child(
                div().flex().flex_col().gap(px(10.)).children([
                    div()
                        .text_center()
                        .text_color(rgb(0xE6E1E5))
                        .text_xl()
                        .child("Non-continuous"),
                    div()
                        .flex()
                        .justify_center()
                        .items_center()
                        .size_112()
                        .border_1()
                        .border_color(border_color)
                        .child(
                            Button::new("btn-1")
                                .top(right_btn_pos.y)
                                .left(right_btn_pos.x)
                                .child("Click me!")
                                .on_click(move |_event, _window, cx| {
                                    right_transition.update(cx, |this, cx| {
                                        let new_position = point(px(0.), px(-100.));
                                        *this = new_position;
                                        cx.notify();
                                    });
                                }),
                        ),
                ]),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.set_menus(vec![Menu {
            name: "My GPUI App".into(),
            items: vec![],
        }]);

        let bounds = Bounds::centered(None, size(px(1200.), px(800.)), cx);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    appears_transparent: true,
                    traffic_light_position: Some(point(px(10.), px(10.))),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_window, cx| cx.new(|_cx| Root),
        )
        .unwrap();

        cx.activate(true);
    });
}

mod element_id_ext {
    use gpui::{ElementId, SharedString};

    pub trait ElementIdExt {
        fn with_suffix(&self, suffix: impl Into<SharedString>) -> ElementId;
    }

    impl ElementIdExt for ElementId {
        fn with_suffix(&self, suffix: impl Into<SharedString>) -> ElementId {
            ElementId::NamedChild(Box::new(self.clone()), suffix.into())
        }
    }
}

pub fn ease_out_bounce(t: f32) -> f32 {
    let n1 = 7.5625;
    let d1 = 2.75;

    if t < 1.0 / d1 {
        n1 * t * t
    } else if t < 2.0 / d1 {
        let t = t - 1.5 / d1;
        n1 * t * t + 0.75
    } else if t < 2.5 / d1 {
        let t = t - 2.25 / d1;
        n1 * t * t + 0.9375
    } else {
        let t = t - 2.625 / d1;
        n1 * t * t + 0.984375
    }
}
