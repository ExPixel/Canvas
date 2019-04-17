use cgmath::{
    Rad,
    Deg,
    Point2,
    Vector2,
    Vector3,
    Vector4,
    Matrix3,
    Matrix4,
    Decomposed,
};
use cgmath::prelude::*;

pub use cgmath::Matrix;

pub type Vec2f = Vector2<f32>;
pub type Vec3f = Vector3<f32>;
pub type Vec4f = Vector4<f32>;
pub type Mat3f = Matrix3<f32>;
pub type Mat4f = Matrix4<f32>;
pub type Point2f = Point2<f32>;
pub type Radf = Rad<f32>;
pub type Degf = Deg<f32>;

pub type Decomposed2f = Decomposed<Vector2<f32>, f32>;

pub type Color3 = Vector3<f32>;
pub type Color4 = Vector4<f32>;

#[inline]
pub fn point2f(x: f32, y: f32) -> Point2f {
    Point2 { x, y }
}

#[inline]
pub fn vec2f(x: f32, y: f32) -> Vec2f {
    cgmath::vec2(x, y)
}

#[inline]
pub fn vec3f(x: f32, y: f32, z: f32) -> Vec3f {
    cgmath::vec3(x, y, z)
}

#[inline]
pub fn vec4f(x: f32, y: f32, z: f32, w: f32) -> Vec4f {
    cgmath::vec4(x, y, z, w)
}

#[inline]
pub fn degf(theta: f32) -> Degf {
    Deg(theta)
}

#[inline]
pub fn radf(theta: f32) -> Radf {
    Rad(theta)
}

pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const RED: Color = Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const GREEN: Color = Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const BLUE: Color = Color { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };

    #[inline]
    pub fn rgba(r: u8, g: u8, b: u8, a: f32) -> Color {
        Color {
            r: (r as f32) / 255.0,
            g: (g as f32) / 255.0,
            b: (b as f32) / 255.0,
            a: a,
        }
    }

    #[inline]
    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color::rgba(r, g, b, 1.0)
    }
}

#[inline]
pub fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::rgb(r, g, b)
}

#[inline]
pub fn rgba(r: u8, g: u8, b: u8, a: f32) -> Color {
    Color::rgba(r, g, b, a)
}

#[cfg(target_endian = "big")]
pub const LITTLE_ENDIAN: bool = false;

#[cfg(not(target_endian = "big"))]
pub const LITTLE_ENDIAN: bool = true;

#[cfg(target_endian = "big")]
pub const BIG_ENDIAN: bool = true;

#[cfg(not(target_endian = "big"))]
pub const BIG_ENDIAN: bool = false;

pub fn decomposed2f(scale: f32, rot: f32, disp: Vec2f) -> Decomposed2f {
    Decomposed {
        scale, rot, disp
    }
}
