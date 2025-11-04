use iced::Color;

pub const fn from_rgba8(r: u8, g: u8, b: u8, a: f32) -> Color {
    Color {
        r: r as f32 / 255.0,
        g: g as f32 / 255.0,
        b: b as f32 / 255.0,
        a,
    }
}

pub const fn from_rgb8(r: u8, g: u8, b: u8) -> Color {
    from_rgba8(r, g, b, 1.0)
}

pub trait If
where
    Self: Sized,
{
    fn if_(self, condition: bool, apply: impl FnOnce(Self) -> Self) -> Self;
}

impl<T> If for T {
    fn if_(self, condition: bool, apply: impl FnOnce(Self) -> Self) -> Self {
        if condition {
            apply(self)
        } else {
            self
        }
    }
}

