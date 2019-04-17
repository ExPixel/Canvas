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
    pub const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };

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

#[inline]
pub fn deg2rad(deg: f32) -> f32 {
    const DEG_TO_RAD: f32 = std::f32::consts::PI / 180.0;
    deg * DEG_TO_RAD
}

/// Converts to degrees to radians but stays within the range (-180.0, 180.0]
#[inline]
pub fn deg2rad_h(deg: f32) -> f32 {
    let rad = deg2rad(deg);
    if rad > 180.0 {
        rad - 360.0
    } else {
        rad
    }
}


pub mod transform {
    use super::{ Mat4f, Vec2f, Vec3f, vec2f, vec3f, vec4f, deg2rad_h };
    use cgmath::Matrix4;

    pub fn identity() -> Mat4f {
        Matrix4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }

    pub fn translate(dx: f32, dy: f32) -> Mat4f {
        const DZ: f32 = 0.0;
        Matrix4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
             dx,  dy,  DZ, 1.0,
        )
    }

    /// Generates a 2-dimensional rotation as a 4x4 matrix.
    #[inline]
    pub fn rotation(theta: f32) -> Mat4f {
        let st = theta.sin();
        let ct = theta.cos();
        // rotation about the z-axis.
        Matrix4::new(
            ct, st, 0.0, 0.0,
            -st, ct, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }

    #[inline]
    pub fn rotation_deg(deg: f32) -> Mat4f {
        rotation(deg2rad_h(deg))
    }

    #[inline]
    pub fn scale(scale_x: f32, scale_y: f32) -> Mat4f {
        const SCALE_Z: f32 = 1.0;
        Matrix4::new(
            scale_x, 0.0, 0.0, 0.0,
            0.0, scale_y, 0.0, 0.0,
            0.0, 0.0, SCALE_Z, 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }

    #[inline]
    pub fn apply2(t: Mat4f, v: Vec2f) -> Vec2f {
        let e = apply3(t, vec3f(v.x, v.y, 1.0));
        vec2f(e.x, e.y)
    }

    #[inline]
    pub fn apply3(t: Mat4f, v: Vec3f) -> Vec3f {
        let mut e = vec4f(v.x, v.y, v.z, 1.0);
        e = t * e;
        vec3f(e.x, e.y, e.z)
    }

    pub fn merge(first: Mat4f, second: Mat4f) -> Mat4f {
        second * first
    }

    pub fn merge_all(transforms: &[Mat4f]) -> Mat4f {
        // column major so we have to do it backwards...
        transforms.iter().rev().product()
    }
}

