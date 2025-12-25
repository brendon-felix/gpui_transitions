use std::{
    fmt::Debug,
    ops::{Add, Mul, Sub},
};

use gpui::{
    Bounds, Corners, DevicePixels, Edges, Percentage, Pixels, Point, Radians, Rems, Rgba, Size,
    colors::Colors, px,
};

/// A value which can be interpolated with another value of the same type.
pub trait Lerp {
    /// Defines how a value is calculated from the start and end goal.
    fn lerp(&self, to: &Self, delta: f32) -> Self;
}

macro_rules! float_lerps {
    ( $( $ty:ty ),+ ) => {
        $(
            impl Lerp for $ty {
                fn lerp(&self, to: &Self, delta: f32) -> Self {
                    lerp(*self, *to, delta as $ty)
                }
            }
        )+
    };
}

float_lerps!(f32, f64);

macro_rules! int_lerps {
    ( $( $ty:ident as $ty_into:ident ),+ ) => {
        $(
            impl Lerp for $ty {
                fn lerp(&self, to: &Self, delta: f32) -> Self {
                    lerp(*self as $ty_into, *to as $ty_into, delta as $ty_into) as $ty
                }
            }
        )+
    };
}

int_lerps!(
    usize as f32,
    u8 as f32,
    u16 as f32,
    u32 as f32,
    u64 as f64,
    u128 as f64,
    isize as f32,
    i8 as f32,
    i16 as f32,
    i32 as f32,
    i64 as f64,
    i128 as f64
);

macro_rules! struct_lerps {
    ( $( $ty:ident $( < $gen:ident > )? { $( $n:ident ),+ } ),+ $(,)? ) => {
        $(
            impl$(<$gen: Lerp + Clone + Debug + Default + PartialEq>)? Lerp for $ty$(<$gen>)? {
                fn lerp(&self, to: &Self, delta: f32) -> Self {
                    $ty$(::<$gen>)? {
                        $(
                            $n: self.$n.lerp(&to.$n, delta)
                        ),+
                    }
                }
            }
        )+
    };
}

struct_lerps!(
    Point<T> { x, y },
    Size<T> { width, height },
    Edges<T> { top, right, bottom, left },
    Corners<T> { top_left, top_right, bottom_right, bottom_left },
    Bounds<T> { origin, size },
    Rgba { r, g, b, a },
    Colors { text, selected_text, background, disabled, selected, border, separator, container }
);

macro_rules! tuple_struct_lerps {
    ( $( $ty:ident ( $n:ty ) ),+ ) => {
        $(
            impl Lerp for $ty {
                fn lerp(&self, to: &Self, delta: f32) -> Self {
                    $ty(self.0.lerp(&to.0, delta))
                }
            }
        )+
    };
}

tuple_struct_lerps!(Radians(f32), Percentage(f32), DevicePixels(i32), Rems(f32));

impl Lerp for Pixels {
    fn lerp(&self, to: &Self, delta: f32) -> Self {
        px((self.to_f64() as f32).lerp(&(to.to_f64() as f32), delta))
    }
}

fn lerp<T>(a: T, b: T, t: T) -> T
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    a + (b - a) * t
}
