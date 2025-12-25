use std::time::Duration;

use element_id_ext::ElementIdExt;
use gpui::{
    AnyElement, App, AppContext, Application, Bounds, Context, ElementId, Fill, Hsla, Menu, Rgba,
    TitlebarOptions, Window, WindowBounds, WindowOptions, div, ease_in_out, ease_out_quint, point,
    prelude::*, px, rgb, size,
};
use gpui_transitions::{Lerp, WindowUseTransition};
use palette::{FromColor, Hsl, IntoColor, Mix, Srgb};
use rand::Rng;
use smallvec::SmallVec;

#[derive(IntoElement)]
struct Button {
    id: ElementId,
    children: SmallVec<[AnyElement; 2]>,
}

impl Button {
    fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            children: SmallVec::new(),
        }
    }
}

impl ParentElement for Button {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Button {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        const HOVER_STRENGTH: f32 = 0.3;

        let base_color_transition = window
            .use_keyed_transition(
                self.id.with_suffix("color"),
                cx,
                Duration::from_millis(1000),
                |_window, _cx| Oklab(random_pastel_hex(None)),
            )
            .with_easing(ease_out_quint());

        let hover_transition = window
            .use_keyed_transition(
                self.id.with_suffix("hover"),
                cx,
                Duration::from_millis(300),
                |_window, _cx| 0.,
            )
            .with_easing(ease_in_out);

        let bg_color = base_color_transition.evaluate(window, cx).lerp(
            &Oklab(rgb(0x110F15)),
            *hover_transition.evaluate(window, cx) * HOVER_STRENGTH,
        );

        div()
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
            .on_click(move |_event, _window, cx| {
                base_color_transition.update(cx, |this, cx| {
                    *this = Oklab(random_pastel_hex(Some(rgba_to_hsla(&this.0).h)));
                    cx.notify();
                });
            })
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
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .justify_center()
            .items_center()
            .absolute()
            .bg(rgb(0x110F15))
            .gap(px(20.))
            .p(px(100.))
            .child(Button::new("btn").child("Click me!"))
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.set_menus(vec![Menu {
            name: "My GPUI App".into(),
            items: vec![],
        }]);

        let bounds = Bounds::centered(None, size(px(620.), px(800.)), cx);

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

fn random_pastel_hex(prev_hue: Option<f32>) -> Rgba {
    let mut rng = rand::rng();

    let hue = match prev_hue {
        Some(prev_hue) => loop {
            let hue = rng.random_range(0.0..360.0);
            let diff = (hue - prev_hue).abs() % 360.0; // wrap-around
            if diff > 120.0 && diff < 300.0 {
                break hue;
            }
        },

        None => rng.random_range(0.0..360.0),
    };

    let s = rng.random_range(0.7..0.95);
    let l = rng.random_range(0.7..0.8);

    let rgb_f32: Srgb<f32> = Hsl::new(hue, s, l).into_color();
    let rgb_u8: Srgb<u8> = rgb_f32.into_format();
    let hex = ((rgb_u8.red as u32) << 16) | ((rgb_u8.green as u32) << 8) | (rgb_u8.blue as u32);

    rgb(hex)
}

fn rgba_to_hsla(rgba: &Rgba) -> Hsla {
    let red = rgba.r as f32 / 255.0;
    let green = rgba.g as f32 / 255.0;
    let blue = rgba.b as f32 / 255.0;
    let alpha = rgba.a as f32 / 255.0;

    let max = red.max(green.max(blue));
    let min = red.min(green.min(blue));
    let delta = max - min;

    let lightness = (max + min) / 2.0;

    let saturation = if delta == 0.0 {
        0.0
    } else {
        delta / (1.0 - (2.0 * lightness - 1.0).abs())
    };

    let hue = if delta == 0.0 {
        0.0
    } else if max == red {
        60.0 * (((green - blue) / delta) % 6.0)
    } else if max == green {
        60.0 * (((blue - red) / delta) + 2.0)
    } else {
        60.0 * (((red - green) / delta) + 4.0)
    };
    let hue = if hue < 0.0 { hue + 360.0 } else { hue };

    Hsla {
        h: hue,
        s: saturation,
        l: lightness,
        a: alpha,
    }
}

/// A simple wrapper around Rgba that implements a more perceptual lerp via Oklab.
#[derive(PartialEq, Clone, Copy)]
struct Oklab(Rgba);

impl Lerp for Oklab {
    fn lerp(&self, to: &Self, delta: f32) -> Self {
        let self_srgba = palette::Srgba::new(self.0.r, self.0.g, self.0.b, self.0.a);
        let to_srgba = palette::Srgba::new(to.0.r, to.0.g, to.0.b, to.0.a);

        let self_oklab: palette::Oklab = palette::Oklab::from_color(self_srgba);
        let to_oklab: palette::Oklab = palette::Oklab::from_color(to_srgba);

        let lerped_oklab = self_oklab.mix(to_oklab, delta);
        let lerped_srgba: palette::Srgba<f32> = palette::Srgba::from_color(lerped_oklab);

        Oklab(gpui::Rgba {
            r: lerped_srgba.red,
            g: lerped_srgba.green,
            b: lerped_srgba.blue,
            a: lerped_srgba.alpha,
        })
    }
}

impl Into<Fill> for Oklab {
    fn into(self) -> Fill {
        self.0.into()
    }
}
