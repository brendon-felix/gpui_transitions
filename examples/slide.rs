use std::time::Duration;

use gpui::{
    AnyElement, App, AppContext, Application, Bounds, Context, Div, ElementId, Interactivity,
    KeyBinding, Menu, StyleRefinement, TitlebarOptions, Window, WindowBounds, WindowOptions,
    actions, div, ease_in_out, ease_out_quint, point, prelude::*, px, rgb, size,
};
use gpui_transitions::{Lerp, WindowUseTransition};
use smallvec::SmallVec;

actions!(app, [Quit]);

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
        const HOVER_STRENGTH: f32 = 0.4;

        let base_color = gpui::white().to_rgb();

        let hover_transition = window
            .use_keyed_transition(
                (self.id.clone(), "hover"),
                cx,
                Duration::from_millis(200),
                |_window, _cx| 0.,
            )
            .with_easing(ease_in_out);

        let bg_color = base_color.lerp(
            &rgb(0x110F15),
            *hover_transition.evaluate(window, cx) * HOVER_STRENGTH,
        );

        self.base
            .id(self.id)
            .cursor_pointer()
            .rounded_full()
            .px_3()
            .py_2()
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

enum Location {
    Top,
    Bottom,
    Left,
    Right,
}

impl Location {
    fn position(&self) -> gpui::Point<gpui::Pixels> {
        match self {
            Location::Top => point(px(0.), px(-100.)),
            Location::Bottom => point(px(0.), px(100.)),
            Location::Left => point(px(-100.), px(0.)),
            Location::Right => point(px(100.), px(0.)),
        }
    }

    fn next(&self) -> Self {
        match self {
            Location::Top => Location::Right,
            Location::Right => Location::Bottom,
            Location::Bottom => Location::Left,
            Location::Left => Location::Top,
        }
    }
}

struct Root {
    button1_location: Location,
    button2_toggle: bool,
}

impl Root {
    fn new() -> Self {
        Self {
            button1_location: Location::Top,
            button2_toggle: false,
        }
    }
}

impl Render for Root {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let slide_transition1 = window
            .use_keyed_transition("slide", cx, Duration::from_millis(500), |_window, _cx| {
                self.button1_location.position()
            })
            .with_easing(ease_out_quint());

        let slide_transition2 = window
            .use_keyed_transition("bounce", cx, Duration::from_millis(800), |_window, _cx| {
                Location::Top.position()
            })
            .with_easing(ease_out_bounce);

        let left_btn_pos = *slide_transition1.evaluate(window, cx);
        let right_btn_pos = *slide_transition2.evaluate(window, cx);

        let border_color = gpui::rgb(0x444054);

        div()
            .size_full()
            .flex()
            .justify_center()
            .items_center()
            .bg(rgb(0x110F15))
            .gap(px(20.))
            .p(px(50.))
            .child(
                div().flex().flex_col().gap(px(10.)).children([
                    div()
                        .text_center()
                        .text_color(rgb(0xE6E1E5))
                        .text_xl()
                        .child("Slide"),
                    div()
                        .flex()
                        .justify_center()
                        .items_center()
                        .size_96()
                        .border_1()
                        .border_color(border_color)
                        .child(
                            Button::new("btn-1")
                                .top(left_btn_pos.y)
                                .left(left_btn_pos.x)
                                .child("Click me!")
                                .on_click(cx.listener(move |root, _event, _window, cx| {
                                    root.button1_location = root.button1_location.next();
                                    slide_transition1.update(cx, |pos, cx| {
                                        *pos = root.button1_location.position();
                                        cx.notify();
                                    });
                                })),
                        ),
                ]),
            )
            .child(
                div().flex().flex_col().gap(px(10.)).children([
                    div()
                        .text_center()
                        .text_color(rgb(0xE6E1E5))
                        .text_xl()
                        .child("Bounce"),
                    div()
                        .flex()
                        .justify_center()
                        .items_center()
                        .size_96()
                        .border_1()
                        .border_color(border_color)
                        .child(
                            Button::new("btn-2")
                                .top(right_btn_pos.y)
                                .left(right_btn_pos.x)
                                .when_else(
                                    self.button2_toggle,
                                    |this| this.child("  Reset  "),
                                    |this| this.child("Click me!"),
                                )
                                .on_click(cx.listener(move |root, _event, _window, cx| {
                                    if root.button2_toggle {
                                        root.button2_toggle = false;
                                        slide_transition2.reset(cx);
                                    } else {
                                        root.button2_toggle = true;
                                        slide_transition2.update(cx, |pos, cx| {
                                            *pos = Location::Bottom.position();
                                            cx.notify();
                                        });
                                    }
                                })),
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

        let bounds = Bounds::centered(None, size(px(1024.), px(768.)), cx);

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
            |_window, cx| cx.new(|_cx| Root::new()),
        )
        .unwrap();

        cx.on_action(|_: &Quit, cx: &mut App| cx.quit());
        cx.bind_keys([
            #[cfg(target_os = "macos")]
            KeyBinding::new("cmd-q", Quit, None),
            KeyBinding::new("alt-f4", Quit, None),
        ]);
        cx.on_window_closed(|cx| cx.quit()).detach();

        cx.activate(true);
    });
}
