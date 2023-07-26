use thiserror::Error;

use crate::Error;

#[cfg(feature = "impl")]
use nalgebra as na;

#[cfg(feature = "impl")]
use colors_transform::{Color, Hsl, Rgb};

#[cfg(feature = "impl")]
use std::ops::{Add, Div, Mul, Neg, Sub};

#[cfg(feature = "mlua")]
use mlua::prelude::*;

#[cfg(feature = "impl")]
macro_rules! impl_vector_methods {
    (for $vec_t:ty where $scalar_t:ty) => {
        pub fn magnitude(&self) -> $scalar_t {
            self.into_alg().magnitude()
        }

        pub fn unit(&self) -> Self {
            self.into_alg().normalize().into()
        }
    };
}

#[cfg(feature = "impl")]
macro_rules! impl_vector_ops {
    (for $vec_t:ty where $scalar_t:ty) => {
        impl Add for $vec_t {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                (self.into_alg() + rhs.into_alg()).into()
            }
        }

        impl Sub for $vec_t {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                (self.into_alg() - rhs.into_alg()).into()
            }
        }

        impl Mul<Self> for $vec_t {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self::Output {
                (self.into_alg().component_mul(&rhs.into_alg())).into()
            }
        }

        impl Mul<$scalar_t> for $vec_t {
            type Output = Self;
            fn mul(self, rhs: $scalar_t) -> Self::Output {
                (self.into_alg() * rhs).into()
            }
        }

        impl Div<Self> for $vec_t {
            type Output = Self;
            fn div(self, rhs: Self) -> Self::Output {
                (self.into_alg().component_div(&rhs.into_alg())).into()
            }
        }

        impl Div<$scalar_t> for $vec_t {
            type Output = Self;
            fn div(self, rhs: $scalar_t) -> Self::Output {
                (self.into_alg() / rhs).into()
            }
        }

        impl Neg for $vec_t {
            type Output = Self;
            fn neg(self) -> Self::Output {
                (-self.into_alg()).into()
            }
        }
    };
}

/// Represents any Roblox enum value.
///
/// Roblox enums are not strongly typed, so the meaning of a value depends on
/// where they're assigned.
///
/// A list of all enums and their values are available [on the Roblox Developer
/// Hub](https://developer.roblox.com/en-us/api-reference/enum).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
pub struct Enum {
    value: u32,
}

impl Enum {
    pub fn from_u32(value: u32) -> Self {
        Self { value }
    }

    pub fn to_u32(self) -> u32 {
        self.value
    }
}

#[cfg(feature = "mlua")]
impl<'lua> FromLua<'lua> for Enum {
    fn from_lua(value: LuaValue<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        Ok(Enum::from_u32(u32::from_lua(value, lua)?))
    }
}

#[cfg(feature = "mlua")]
impl<'lua> IntoLua<'lua> for Enum {
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        self.to_u32().into_lua(lua)
    }
}

/// The standard 2D vector type used in Roblox.
///
/// ## See Also
/// * [`Vector2int16`][struct.Vector2int16.html]
/// * [Vector2 on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/Vector2)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    #[cfg(feature = "impl")]
    fn into_alg(self) -> na::Vector2<f32> {
        self.into()
    }

    #[cfg(feature = "impl")]
    impl_vector_methods! {for Vector2 where f32}
}

#[cfg(feature = "impl")]
impl From<na::Vector2<f32>> for Vector2 {
    fn from(value: na::Vector2<f32>) -> Self {
        Self::new(value.x, value.y)
    }
}

#[cfg(feature = "impl")]
impl From<Vector2> for na::Vector2<f32> {
    fn from(value: Vector2) -> Self {
        Self::new(value.x, value.y)
    }
}

#[cfg(feature = "impl")]
impl_vector_ops! {for Vector2 where f32}

#[cfg(feature = "mlua")]
impl<'lua> FromLua<'lua> for Vector2 {
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        let LuaValue::UserData(value) = value else {
            return Err(LuaError::UserDataTypeMismatch);
        };
        if !value.is::<Self>() {
            return Err(LuaError::UserDataTypeMismatch);
        }
        Ok(Self::new(value.get("X")?, value.get("Y")?))
    }
}

#[cfg(feature = "mlua")]
impl LuaUserData for Vector2 {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("X", |_lua, this| Ok(this.x));
        fields.add_field_method_get("Y", |_lua, this| Ok(this.y));
        fields.add_field_method_get("Magnitude", |_lua, this| Ok(this.magnitude()));
        fields.add_field_method_get("Unit", |_lua, this| Ok(this.unit()));
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::Add, |_lua, &this, rhs: Self| Ok(this + rhs));
        methods.add_meta_method(LuaMetaMethod::Sub, |_lua, &this, rhs: Self| Ok(this - rhs));
        methods.add_meta_method(LuaMetaMethod::Unm, |_lua, &this, ()| Ok(-this));
        methods.add_meta_method(LuaMetaMethod::Mul, |lua, &this, rhs: LuaValue| match rhs {
            LuaValue::UserData(_) => Ok(this * Self::from_lua(rhs, lua)?),
            LuaValue::Number(num) => Ok(this * num as f32),
            _ => Err(LuaError::MetaMethodTypeError {
                method: LuaMetaMethod::Mul.to_string(),
                type_name: rhs.type_name(),
                message: Some("expected Vector or number".to_string()),
            }),
        });
        methods.add_meta_method(LuaMetaMethod::Div, |lua, &this, rhs: LuaValue| match rhs {
            LuaValue::UserData(_) => Ok(this / Self::from_lua(rhs, lua)?),
            LuaValue::Number(num) => Ok(this / num as f32),
            _ => Err(LuaError::MetaMethodTypeError {
                method: LuaMetaMethod::Div.to_string(),
                type_name: rhs.type_name(),
                message: Some("expected Vector or number".to_string()),
            }),
        });
    }
}

/// A version of [`Vector2`][Vector2] whose coordinates are signed 16-bit
/// integers.
///
/// ## See Also
/// * [`Vector2`][Vector2], which is used for most values.
/// * [Vector2int16 on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/Vector2int16)
///
/// [Vector2]: struct.Vector2.html
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vector2int16 {
    pub x: i16,
    pub y: i16,
}

impl Vector2int16 {
    pub fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    #[cfg(feature = "impl")]
    fn into_alg(self) -> na::Vector2<i16> {
        self.into()
    }
}

#[cfg(feature = "impl")]
impl From<na::Vector2<i16>> for Vector2int16 {
    fn from(value: na::Vector2<i16>) -> Self {
        Self::new(value.x, value.y)
    }
}

#[cfg(feature = "impl")]
impl From<Vector2int16> for na::Vector2<i16> {
    fn from(value: Vector2int16) -> Self {
        Self::new(value.x, value.y)
    }
}

#[cfg(feature = "impl")]
impl_vector_ops! {for Vector2int16 where i16}

#[cfg(feature = "mlua")]
impl<'lua> FromLua<'lua> for Vector2int16 {
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        let LuaValue::UserData(value) = value else {
            return Err(LuaError::UserDataTypeMismatch);
        };
        if !value.is::<Self>() {
            return Err(LuaError::UserDataTypeMismatch);
        }
        Ok(Self::new(value.get("X")?, value.get("Y")?))
    }
}

#[cfg(feature = "mlua")]
impl LuaUserData for Vector2int16 {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("X", |_lua, this| Ok(this.x));
        fields.add_field_method_get("Y", |_lua, this| Ok(this.y));
        fields.add_field_method_get("x", |_lua, this| Ok(this.x));
        fields.add_field_method_get("y", |_lua, this| Ok(this.y));
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::Add, |_lua, &this, rhs: Self| Ok(this + rhs));
        methods.add_meta_method(LuaMetaMethod::Sub, |_lua, &this, rhs: Self| Ok(this - rhs));
        methods.add_meta_method(LuaMetaMethod::Unm, |_lua, &this, ()| Ok(-this));
        methods.add_meta_method(LuaMetaMethod::Mul, |lua, &this, rhs: LuaValue| match rhs {
            LuaValue::UserData(_) => Ok(this * Self::from_lua(rhs, lua)?),
            LuaValue::Number(num) => Ok(this * num as i16),
            _ => Err(LuaError::MetaMethodTypeError {
                method: LuaMetaMethod::Mul.to_string(),
                type_name: rhs.type_name(),
                message: Some("expected Vector or number".to_string()),
            }),
        });
        methods.add_meta_method(LuaMetaMethod::Div, |lua, &this, rhs: LuaValue| match rhs {
            LuaValue::UserData(_) => Ok(this / Self::from_lua(rhs, lua)?),
            LuaValue::Number(num) => Ok(this / num as i16),
            _ => Err(LuaError::MetaMethodTypeError {
                method: LuaMetaMethod::Div.to_string(),
                type_name: rhs.type_name(),
                message: Some("expected Vector or number".to_string()),
            }),
        });
    }
}

/// The standard 3D vector type used in Roblox.
///
/// ## See Also
/// * [`Vector3int16`][struct.Vector3int16.html]
/// * [Vector3 on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/Vector3)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

fn approx_unit_or_zero(value: f32) -> Option<i32> {
    if value.abs() <= std::f32::EPSILON {
        Some(0)
    } else if value.abs() - 1.0 <= std::f32::EPSILON {
        Some(1.0f32.copysign(value) as i32)
    } else {
        None
    }
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// If the vector is a positive or negative basis vector, returns
    /// its corresponding ID. Otherwise, returns None.
    /// The mapping goes like this:
    /// (1.0, 0.0, 0.0) -> 0
    /// (0.0, 1.0, 0.0) -> 1
    /// (0.0, 0.0, 1.0) -> 2
    /// (-1.0, 0.0, 0.0) -> 3
    /// (0.0, -1.0, 0.0) -> 4
    /// (0.0, 0.0, -1.0) -> 5
    // We accidentally did not follow this convention, but that's okay, it's not
    // a huge deal and not something we can change now.
    #[allow(clippy::wrong_self_convention)]
    pub fn to_normal_id(&self) -> Option<u8> {
        fn get_normal_id(position: u8, value: i32) -> Option<u8> {
            match value {
                1 => Some(position),
                -1 => Some(position + 3),
                _ => None,
            }
        }

        let x = approx_unit_or_zero(self.x);
        let y = approx_unit_or_zero(self.y);
        let z = approx_unit_or_zero(self.z);

        match (x, y, z) {
            (Some(x), Some(0), Some(0)) => get_normal_id(0, x),
            (Some(0), Some(y), Some(0)) => get_normal_id(1, y),
            (Some(0), Some(0), Some(z)) => get_normal_id(2, z),
            _ => None,
        }
    }

    #[cfg(feature = "impl")]
    fn into_alg(self) -> na::Vector3<f32> {
        self.into()
    }

    #[cfg(feature = "impl")]
    impl_vector_methods! {for Vector2 where f32}
}

#[cfg(feature = "impl")]
impl From<na::Vector3<f32>> for Vector3 {
    fn from(value: na::Vector3<f32>) -> Self {
        Self::new(value.x, value.y, value.z)
    }
}

#[cfg(feature = "impl")]
impl From<Vector3> for na::Vector3<f32> {
    fn from(value: Vector3) -> Self {
        Self::new(value.x, value.y, value.z)
    }
}

#[cfg(feature = "impl")]
impl From<na::Point3<f32>> for Vector3 {
    fn from(value: na::Point3<f32>) -> Self {
        Self::new(value.x, value.y, value.z)
    }
}

#[cfg(feature = "impl")]
impl From<Vector3> for na::Point3<f32> {
    fn from(value: Vector3) -> Self {
        Self::new(value.x, value.y, value.z)
    }
}

#[cfg(feature = "impl")]
impl From<na::Translation3<f32>> for Vector3 {
    fn from(value: na::Translation3<f32>) -> Self {
        Self::new(value.x, value.y, value.z)
    }
}

#[cfg(feature = "impl")]
impl From<Vector3> for na::Translation3<f32> {
    fn from(value: Vector3) -> Self {
        Self::new(value.x, value.y, value.z)
    }
}

#[cfg(feature = "impl")]
impl_vector_ops! {for Vector3 where f32}

#[cfg(feature = "mlua")]
impl<'lua> FromLua<'lua> for Vector3 {
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        let LuaValue::UserData(value) = value else {
            return Err(LuaError::UserDataTypeMismatch);
        };
        if !value.is::<Self>() {
            return Err(LuaError::UserDataTypeMismatch);
        }
        Ok(Self::new(value.get("X")?, value.get("Y")?, value.get("Z")?))
    }
}

#[cfg(feature = "mlua")]
impl LuaUserData for Vector3 {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("X", |_lua, this| Ok(this.x));
        fields.add_field_method_get("Y", |_lua, this| Ok(this.y));
        fields.add_field_method_get("Z", |_lua, this| Ok(this.z));
        fields.add_field_method_get("x", |_lua, this| Ok(this.x));
        fields.add_field_method_get("y", |_lua, this| Ok(this.y));
        fields.add_field_method_get("z", |_lua, this| Ok(this.z));
        fields.add_field_method_get("Magnitude", |_lua, this| Ok(this.magnitude()));
        fields.add_field_method_get("Unit", |_lua, this| Ok(this.unit()));
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::Add, |_lua, &this, rhs: Self| Ok(this + rhs));
        methods.add_meta_method(LuaMetaMethod::Sub, |_lua, &this, rhs: Self| Ok(this - rhs));
        methods.add_meta_method(LuaMetaMethod::Unm, |_lua, &this, ()| Ok(-this));
        methods.add_meta_method(LuaMetaMethod::Mul, |lua, &this, rhs: LuaValue| match rhs {
            LuaValue::UserData(_) => Ok(this * Self::from_lua(rhs, lua)?),
            LuaValue::Number(num) => Ok(this * num as f32),
            _ => Err(LuaError::MetaMethodTypeError {
                method: LuaMetaMethod::Mul.to_string(),
                type_name: rhs.type_name(),
                message: Some("expected Vector or number".to_string()),
            }),
        });
        methods.add_meta_method(LuaMetaMethod::Div, |lua, &this, rhs: LuaValue| match rhs {
            LuaValue::UserData(_) => Ok(this / Self::from_lua(rhs, lua)?),
            LuaValue::Number(num) => Ok(this / num as f32),
            _ => Err(LuaError::MetaMethodTypeError {
                method: LuaMetaMethod::Div.to_string(),
                type_name: rhs.type_name(),
                message: Some("expected Vector or number".to_string()),
            }),
        });
    }
}

/// A version of [`Vector3`][Vector3] whose coordinates are signed 16-bit
/// integers. `Vector3int16` is often used when working with Terrain.
///
/// ## See Also
/// * [`Vector3`][Vector3], which is used for most values.
/// * [Vector3int16 on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/Vector3int16)
///
/// [Vector3]: struct.Vector3.html
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vector3int16 {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

impl Vector3int16 {
    pub fn new(x: i16, y: i16, z: i16) -> Self {
        Self { x, y, z }
    }

    #[cfg(feature = "impl")]
    fn into_alg(self) -> na::Vector3<i16> {
        self.into()
    }
}

#[cfg(feature = "impl")]
impl From<na::Vector3<i16>> for Vector3int16 {
    fn from(value: na::Vector3<i16>) -> Self {
        Self::new(value.x, value.y, value.z)
    }
}

#[cfg(feature = "impl")]
impl From<Vector3int16> for na::Vector3<i16> {
    fn from(value: Vector3int16) -> Self {
        Self::new(value.x, value.y, value.z)
    }
}

#[cfg(feature = "impl")]
impl_vector_ops! {for Vector3int16 where i16}

#[cfg(feature = "mlua")]
impl<'lua> FromLua<'lua> for Vector3int16 {
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        let LuaValue::UserData(value) = value else {
            return Err(LuaError::UserDataTypeMismatch);
        };
        if !value.is::<Self>() {
            return Err(LuaError::UserDataTypeMismatch);
        }
        Ok(Self::new(value.get("X")?, value.get("Y")?, value.get("Z")?))
    }
}

#[cfg(feature = "mlua")]
impl LuaUserData for Vector3int16 {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("X", |_lua, this| Ok(this.x));
        fields.add_field_method_get("Y", |_lua, this| Ok(this.y));
        fields.add_field_method_get("Z", |_lua, this| Ok(this.z));
        fields.add_field_method_get("x", |_lua, this| Ok(this.x));
        fields.add_field_method_get("y", |_lua, this| Ok(this.y));
        fields.add_field_method_get("z", |_lua, this| Ok(this.z));
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::Add, |_lua, &this, rhs: Self| Ok(this + rhs));
        methods.add_meta_method(LuaMetaMethod::Sub, |_lua, &this, rhs: Self| Ok(this - rhs));
        methods.add_meta_method(LuaMetaMethod::Unm, |_lua, &this, ()| Ok(-this));
        methods.add_meta_method(LuaMetaMethod::Mul, |lua, &this, rhs: LuaValue| match rhs {
            LuaValue::UserData(_) => Ok(this * Self::from_lua(rhs, lua)?),
            LuaValue::Number(num) => Ok(this * num as i16),
            _ => Err(LuaError::MetaMethodTypeError {
                method: LuaMetaMethod::Mul.to_string(),
                type_name: rhs.type_name(),
                message: Some("expected Vector or number".to_string()),
            }),
        });
        methods.add_meta_method(LuaMetaMethod::Div, |lua, &this, rhs: LuaValue| match rhs {
            LuaValue::UserData(_) => Ok(this / Self::from_lua(rhs, lua)?),
            LuaValue::Number(num) => Ok(this / num as i16),
            _ => Err(LuaError::MetaMethodTypeError {
                method: LuaMetaMethod::Div.to_string(),
                type_name: rhs.type_name(),
                message: Some("expected Vector or number".to_string()),
            }),
        });
    }
}

/// Represents a position and orientation in 3D space.
///
/// ## See Also
/// * [CFrame on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/CFrame)
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct CFrame {
    pub position: Vector3,
    pub orientation: Matrix3,
}

impl CFrame {
    pub fn new(position: Vector3, orientation: Matrix3) -> Self {
        Self {
            position,
            orientation,
        }
    }

    #[cfg(feature = "impl")]
    fn into_alg(self) -> na::IsometryMatrix3<f32> {
        self.into()
    }

    #[cfg(feature = "impl")]
    pub fn look_at(at: Vector3, look: Vector3, up: Option<Vector3>) -> Self {
        let up = up.unwrap_or_else(|| Vector3::new(0.0, 1.0, 0.0));
        na::IsometryMatrix3::look_at_rh(&look.into(), &at.into(), &up.into()).into()
    }

    #[cfg(feature = "impl")]
    pub fn from_euler_angles_xyz(rx: f32, ry: f32, rz: f32) -> Self {
        let r = na::Rotation3::from_euler_angles(rx, ry, rz);
        let t = na::Translation3::new(0.0, 0.0, 0.0);
        na::IsometryMatrix3::from_parts(t, r).into()
    }

    #[cfg(feature = "impl")]
    pub fn to_euler_angles_xyz(&self) -> (f32, f32, f32) {
        na::Rotation3::from(self.orientation).euler_angles()
    }

    #[cfg(feature = "impl")]
    pub fn from_euler_angles_yxz(rx: f32, ry: f32, rz: f32) -> Self {
        todo!(
            "from_euler_angles_yxz({}, {}, {}): No implementation provided by nalgebra...",
            rx,
            ry,
            rz
        )
    }

    #[cfg(feature = "impl")]
    pub fn to_euler_angles_yxz(&self) -> (f32, f32, f32) {
        todo!("to_euler_angles_yxz(): No implementation provided by nalgebra...")
    }

    #[cfg(feature = "impl")]
    pub fn from_axis_angle(v: Vector3, r: f32) -> Self {
        let v = na::UnitVector3::new_normalize(v.into());
        let r = na::Rotation3::from_axis_angle(&v, r);
        let t = na::Translation3::new(0.0, 0.0, 0.0);
        na::IsometryMatrix3::from_parts(t, r).into()
    }

    #[cfg(feature = "impl")]
    pub fn to_axis_angle(&self) -> (Vector3, f32) {
        let (v, r) = na::Rotation3::from(self.orientation)
            .axis_angle()
            .expect("axis angle should not be zero");
        (v.into_inner().into(), r)
    }

    #[cfg(feature = "impl")]
    pub fn from_matrix(pos: Vector3, vx: Vector3, vy: Vector3, vz: Option<Vector3>) -> Self {
        let vx: na::Vector3<_> = vx.into();
        let vy: na::Vector3<_> = vy.into();
        let vz: na::Vector3<_> = vz.map_or_else(|| vx.cross(&vy).normalize(), |v| v.into());
        let r = na::Rotation3::from_matrix(&na::Matrix3::from_columns(&[vx, vy, vz]));
        na::IsometryMatrix3::from_parts(pos.into(), r).into()
    }

    #[cfg(feature = "impl")]
    pub fn rotation(&self) -> Self {
        Self::new(Vector3::new(0.0, 0.0, 0.0), self.orientation)
    }

    #[cfg(feature = "impl")]
    pub fn right_vector(&self) -> Vector3 {
        let o = &self.orientation;
        Vector3::new(o.x.x, o.y.x, o.z.x)
    }

    #[cfg(feature = "impl")]
    pub fn up_vector(&self) -> Vector3 {
        let o = &self.orientation;
        Vector3::new(o.x.y, o.y.y, o.z.y)
    }

    #[cfg(feature = "impl")]
    pub fn look_vector(&self) -> Vector3 {
        let o = &self.orientation;
        Vector3::new(-o.x.z, -o.y.z, -o.z.z)
    }

    #[cfg(feature = "impl")]
    pub fn components(&self) -> (f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32) {
        let p = &self.position;
        let o = &self.orientation;
        (
            p.x, p.y, p.z, o.x.x, o.x.y, o.x.z, o.y.x, o.y.y, o.y.z, o.z.x, o.z.y, o.z.z,
        )
    }

    #[cfg(feature = "impl")]
    pub fn inverse(&self) -> Self {
        self.into_alg().inverse().into()
    }

    #[cfg(feature = "impl")]
    pub fn orthonormalize(&self) -> Self {
        Self::new(self.position, self.orientation.orthonormalize())
    }

    #[cfg(feature = "impl")]
    pub fn lerp(&self, goal: Self, alpha: f32) -> Self {
        if alpha == 0.0 {
            self.clone()
        } else if alpha == 1.0 {
            goal
        } else {
            let q1 = na::UnitQuaternion::from(self.orientation);
            let q2 = na::UnitQuaternion::from(goal.orientation);
            let q = q1.slerp(&q2, alpha);
            let t = self
                .position
                .into_alg()
                .lerp(&goal.position.into_alg(), alpha);
            na::IsometryMatrix3::from_parts(t.into(), q.into()).into()
        }
    }

    #[cfg(feature = "impl")]
    pub fn to_world_space(&self, cf: CFrame) -> Self {
        *self * cf
    }

    #[cfg(feature = "impl")]
    pub fn to_object_space(&self, cf: CFrame) -> Self {
        self.inverse() * cf
    }

    #[cfg(feature = "impl")]
    pub fn point_to_world_space(&self, v3: Vector3) -> Vector3 {
        *self * v3
    }

    #[cfg(feature = "impl")]
    pub fn point_to_object_space(&self, v3: Vector3) -> Vector3 {
        self.inverse() * v3
    }

    #[cfg(feature = "impl")]
    pub fn vector_to_world_space(&self, v3: Vector3) -> Vector3 {
        (*self - self.position) * v3
    }
    #[cfg(feature = "impl")]
    pub fn vector_to_object_space(&self, v3: Vector3) -> Vector3 {
        (self.inverse() - self.inverse().position) * v3
    }
}

#[cfg(feature = "impl")]
impl From<na::IsometryMatrix3<f32>> for CFrame {
    fn from(value: na::IsometryMatrix3<f32>) -> Self {
        Self::new(value.translation.into(), value.rotation.into())
    }
}

#[cfg(feature = "impl")]
impl From<CFrame> for na::IsometryMatrix3<f32> {
    fn from(value: CFrame) -> Self {
        Self::from_parts(value.position.into(), value.orientation.into())
    }
}

#[cfg(feature = "impl")]
impl Add<Vector3> for CFrame {
    type Output = Self;
    fn add(self, rhs: Vector3) -> Self::Output {
        let pos = self.position;
        let translated = Vector3::new(pos.x + rhs.x, pos.y + rhs.y, pos.z + rhs.z);
        CFrame::new(translated, self.orientation)
    }
}

#[cfg(feature = "impl")]
impl Sub<Vector3> for CFrame {
    type Output = Self;
    fn sub(self, rhs: Vector3) -> Self::Output {
        let pos = self.position;
        let translated = Vector3::new(pos.x - rhs.x, pos.y - rhs.y, pos.z - rhs.z);
        CFrame::new(translated, self.orientation)
    }
}

#[cfg(feature = "impl")]
impl Mul<Self> for CFrame {
    type Output = CFrame;
    fn mul(self, rhs: Self) -> Self::Output {
        (self.into_alg() * rhs.into_alg()).into()
    }
}

#[cfg(feature = "impl")]
impl Mul<Vector3> for CFrame {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Self::Output {
        (na::Matrix3::from(self.orientation).transpose() * rhs.into_alg()).into()
    }
}

#[cfg(feature = "mlua")]
impl<'lua> FromLua<'lua> for CFrame {
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        let LuaValue::UserData(value) = value else {
            return Err(LuaError::UserDataTypeMismatch);
        };
        if !value.is::<Self>() {
            return Err(LuaError::UserDataTypeMismatch);
        }
        Ok(Self::new(
            value.get("Position")?,
            Matrix3::new(
                value.get("XVector")?,
                value.get("YVector")?,
                value.get("ZVector")?,
            ),
        ))
    }
}

#[cfg(feature = "mlua")]
impl LuaUserData for CFrame {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("Position", |_lua, this| Ok(this.position));
        fields.add_field_method_get("XVector", |_lua, this| Ok(this.orientation.x));
        fields.add_field_method_get("YVector", |_lua, this| Ok(this.orientation.y));
        fields.add_field_method_get("ZVector", |_lua, this| Ok(this.orientation.z));
        fields.add_field_method_get("Rotation", |_lua, this| Ok(this.rotation()));
        fields.add_field_method_get("RightVector", |_lua, this| Ok(this.right_vector()));
        fields.add_field_method_get("UpVector", |_lua, this| Ok(this.up_vector()));
        fields.add_field_method_get("LookVector", |_lua, this| Ok(this.look_vector()));
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::Add, |_lua, &this, rhs: Vector3| {
            Ok(this + rhs)
        });
        methods.add_meta_method(LuaMetaMethod::Sub, |_lua, &this, rhs: Vector3| {
            Ok(this - rhs)
        });
        methods.add_meta_method(LuaMetaMethod::Mul, |lua, &this, rhs: LuaValue| {
            let type_err = Err(LuaError::MetaMethodTypeError {
                method: LuaMetaMethod::Mul.to_string(),
                type_name: rhs.type_name(),
                message: Some("expected Vector3 or CFrame".to_string()),
            });
            let LuaValue::UserData(ref other) = rhs else { return type_err; };
            if other.is::<CFrame>() {
                (this * CFrame::from_lua(rhs, lua)?).into_lua(lua)
            } else if other.is::<Vector3>() {
                (this * Vector3::from_lua(rhs, lua)?).into_lua(lua)
            } else {
                type_err
            }
        });

        methods.add_method("Inverse", |_lua, this, ()| Ok(this.inverse()));
        methods.add_method("Lerp", |_lua, this, (goal, alpha): (CFrame, f32)| {
            Ok(this.lerp(goal, alpha))
        });
        methods.add_method("ToWorldSpace", |_lua, this, cf: CFrame| {
            Ok(this.to_world_space(cf))
        });
        methods.add_method("ToObjectSpace", |_lua, this, cf: CFrame| {
            Ok(this.to_object_space(cf))
        });
        methods.add_method("PointToWorldSpace", |_lua, this, v3: Vector3| {
            Ok(this.point_to_world_space(v3))
        });
        methods.add_method("PointToObjectSpace", |_lua, this, v3: Vector3| {
            Ok(this.point_to_object_space(v3))
        });
        methods.add_method("VectorToWorldSpace", |_lua, this, v3: Vector3| {
            Ok(this.vector_to_world_space(v3))
        });
        methods.add_method("VectorToObjectSpace", |_lua, this, v3: Vector3| {
            Ok(this.vector_to_object_space(v3))
        });
        methods.add_method("GetComponents", |_lua, this, ()| Ok(this.components()));
        methods.add_method("ToEulerAnglesXYZ", |_lua, this, ()| {
            Ok(this.to_euler_angles_xyz())
        });
        methods.add_method("ToEulerAnglesYXZ", |_lua, this, ()| {
            Ok(this.to_euler_angles_yxz())
        });
        methods.add_method("ToOrientation", |_lua, this, ()| {
            Ok(this.to_euler_angles_yxz())
        });
        methods.add_method("ToAxisAngle", |_lua, this, ()| Ok(this.to_axis_angle()));
    }
}

/// Used to represent the `orientation` field of `CFrame` and not a standalone
/// type in Roblox.
///
/// Internally represented in row-major, i.e., each Vector3 represents a row.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix3 {
    pub x: Vector3,
    pub y: Vector3,
    pub z: Vector3,
}

#[derive(Debug, Error)]
pub(crate) enum Matrix3Error {
    #[error("invalid rotation ID: {id}")]
    BadRotationId { id: u8 },
}

impl Matrix3 {
    pub fn new(x: Vector3, y: Vector3, z: Vector3) -> Self {
        Self { x, y, z }
    }

    pub fn identity() -> Self {
        Self {
            x: Vector3::new(1.0, 0.0, 0.0),
            y: Vector3::new(0.0, 1.0, 0.0),
            z: Vector3::new(0.0, 0.0, 1.0),
        }
    }

    pub fn transpose(&self) -> Self {
        Self {
            x: Vector3::new(self.x.x, self.y.x, self.z.x),
            y: Vector3::new(self.x.y, self.y.y, self.z.y),
            z: Vector3::new(self.x.z, self.y.z, self.z.z),
        }
    }

    pub fn to_basic_rotation_id(&self) -> Option<u8> {
        let transpose = self.transpose();
        let x_id = transpose.x.to_normal_id()?;
        let y_id = transpose.y.to_normal_id()?;
        let z_id = transpose.z.to_normal_id()?;
        let basic_rotation_id = (6 * x_id) + y_id + 1;

        // Because we don't enforce orthonormality, it's still possible at
        // this point for the z row to differ from the basic rotation's z
        // row. We check for this case to avoid altering the value.
        if Matrix3::from_basic_rotation_id(basic_rotation_id)
            .ok()?
            .transpose()
            .z
            .to_normal_id()?
            == z_id
        {
            Some(basic_rotation_id)
        } else {
            None
        }
    }

    pub fn from_basic_rotation_id(id: u8) -> Result<Matrix3, Error> {
        match id {
            0x02 => Ok(Matrix3::identity()),
            0x03 => Ok(Matrix3::new(
                Vector3::new(1.0, 0.0, 0.0),
                Vector3::new(0.0, 0.0, -1.0),
                Vector3::new(0.0, 1.0, 0.0),
            )),
            0x05 => Ok(Matrix3::new(
                Vector3::new(1.0, 0.0, 0.0),
                Vector3::new(0.0, -1.0, 0.0),
                Vector3::new(0.0, 0.0, -1.0),
            )),
            0x06 => Ok(Matrix3::new(
                Vector3::new(1.0, 0.0, 0.0),
                Vector3::new(0.0, 0.0, 1.0),
                Vector3::new(0.0, -1.0, 0.0),
            )),
            0x07 => Ok(Matrix3::new(
                Vector3::new(0.0, 1.0, 0.0),
                Vector3::new(1.0, 0.0, 0.0),
                Vector3::new(0.0, 0.0, -1.0),
            )),
            0x09 => Ok(Matrix3::new(
                Vector3::new(0.0, 0.0, 1.0),
                Vector3::new(1.0, 0.0, 0.0),
                Vector3::new(0.0, 1.0, 0.0),
            )),
            0x0a => Ok(Matrix3::new(
                Vector3::new(0.0, -1.0, 0.0),
                Vector3::new(1.0, 0.0, 0.0),
                Vector3::new(0.0, 0.0, 1.0),
            )),
            0x0c => Ok(Matrix3::new(
                Vector3::new(0.0, 0.0, -1.0),
                Vector3::new(1.0, 0.0, 0.0),
                Vector3::new(0.0, -1.0, 0.0),
            )),
            0x0d => Ok(Matrix3::new(
                Vector3::new(0.0, 1.0, 0.0),
                Vector3::new(0.0, 0.0, 1.0),
                Vector3::new(1.0, 0.0, 0.0),
            )),
            0x0e => Ok(Matrix3::new(
                Vector3::new(0.0, 0.0, -1.0),
                Vector3::new(0.0, 1.0, 0.0),
                Vector3::new(1.0, 0.0, 0.0),
            )),
            0x10 => Ok(Matrix3::new(
                Vector3::new(0.0, -1.0, 0.0),
                Vector3::new(0.0, 0.0, -1.0),
                Vector3::new(1.0, 0.0, 0.0),
            )),
            0x11 => Ok(Matrix3::new(
                Vector3::new(0.0, 0.0, 1.0),
                Vector3::new(0.0, -1.0, 0.0),
                Vector3::new(1.0, 0.0, 0.0),
            )),
            0x14 => Ok(Matrix3::new(
                Vector3::new(-1.0, 0.0, 0.0),
                Vector3::new(0.0, 1.0, 0.0),
                Vector3::new(0.0, 0.0, -1.0),
            )),
            0x15 => Ok(Matrix3::new(
                Vector3::new(-1.0, 0.0, 0.0),
                Vector3::new(0.0, 0.0, 1.0),
                Vector3::new(0.0, 1.0, 0.0),
            )),
            0x17 => Ok(Matrix3::new(
                Vector3::new(-1.0, 0.0, 0.0),
                Vector3::new(0.0, -1.0, 0.0),
                Vector3::new(0.0, 0.0, 1.0),
            )),
            0x18 => Ok(Matrix3::new(
                Vector3::new(-1.0, 0.0, 0.0),
                Vector3::new(0.0, 0.0, -1.0),
                Vector3::new(0.0, -1.0, 0.0),
            )),
            0x19 => Ok(Matrix3::new(
                Vector3::new(0.0, 1.0, 0.0),
                Vector3::new(-1.0, 0.0, 0.0),
                Vector3::new(0.0, 0.0, 1.0),
            )),
            0x1b => Ok(Matrix3::new(
                Vector3::new(0.0, 0.0, -1.0),
                Vector3::new(-1.0, 0.0, 0.0),
                Vector3::new(0.0, 1.0, 0.0),
            )),
            0x1c => Ok(Matrix3::new(
                Vector3::new(0.0, -1.0, 0.0),
                Vector3::new(-1.0, 0.0, 0.0),
                Vector3::new(0.0, 0.0, -1.0),
            )),
            0x1e => Ok(Matrix3::new(
                Vector3::new(0.0, 0.0, 1.0),
                Vector3::new(-1.0, 0.0, 0.0),
                Vector3::new(0.0, -1.0, 0.0),
            )),
            0x1f => Ok(Matrix3::new(
                Vector3::new(0.0, 1.0, 0.0),
                Vector3::new(0.0, 0.0, -1.0),
                Vector3::new(-1.0, 0.0, 0.0),
            )),
            0x20 => Ok(Matrix3::new(
                Vector3::new(0.0, 0.0, 1.0),
                Vector3::new(0.0, 1.0, 0.0),
                Vector3::new(-1.0, 0.0, 0.0),
            )),
            0x22 => Ok(Matrix3::new(
                Vector3::new(0.0, -1.0, 0.0),
                Vector3::new(0.0, 0.0, 1.0),
                Vector3::new(-1.0, 0.0, 0.0),
            )),
            0x23 => Ok(Matrix3::new(
                Vector3::new(0.0, 0.0, -1.0),
                Vector3::new(0.0, -1.0, 0.0),
                Vector3::new(-1.0, 0.0, 0.0),
            )),
            _ => Err(Error::from(Matrix3Error::BadRotationId { id })),
        }
    }

    #[cfg(feature = "impl")]
    pub(super) fn orthonormalize(&self) -> Self {
        let (mut e00, mut e01, mut e02) = (self.x.x, self.x.y, self.x.z);
        let (mut e10, mut e11, mut e12) = (self.y.x, self.y.y, self.y.z);
        let (mut e20, mut e21, mut e22) = (self.z.x, self.z.y, self.z.z);

        // Algorithm uses Gram-Schmidt orthogonalization.  If 'this' matrix is
        // M = [m0|m1|m2], then orthonormal output matrix is Q = [q0|q1|q2],
        //
        //   q0 = m0/|m0|
        //   q1 = (m1-(q0*m1)q0)/|m1-(q0*m1)q0|
        //   q2 = (m2-(q0*m2)q0-(q1*m2)q1)/|m2-(q0*m2)q0-(q1*m2)q1|
        //
        // where |V| indicates length of vector V and A*B indicates dot
        // product of vectors A and B.

        // compute q0
        let f_inv_length = 1.0 / (e00 * e00 + e10 * e10 + e20 * e20).sqrt();

        e00 *= f_inv_length;
        e10 *= f_inv_length;
        e20 *= f_inv_length;

        // compute q1
        let f_dot0 = e00 * e01 + e10 * e11 + e20 * e21;

        e01 -= f_dot0 * e00;
        e11 -= f_dot0 * e10;
        e21 -= f_dot0 * e20;

        let f_inv_length = 1.0 / (e01 * e01 + e11 * e11 + e21 * e21).sqrt();

        e01 *= f_inv_length;
        e11 *= f_inv_length;
        e21 *= f_inv_length;

        // compute q2
        let f_dot1 = e01 * e02 + e11 * e12 + e21 * e22;

        let f_dot0 = e00 * e02 + e10 * e12 + e20 * e22;

        e02 -= f_dot0 * e00 + f_dot1 * e01;
        e12 -= f_dot0 * e10 + f_dot1 * e11;
        e22 -= f_dot0 * e20 + f_dot1 * e21;

        let f_inv_length = 1.0 / (e02 * e02 + e12 * e12 + e22 * e22).sqrt();

        e02 *= f_inv_length;
        e12 *= f_inv_length;
        e22 *= f_inv_length;

        Self::new(
            Vector3::new(e00, e01, e02),
            Vector3::new(e10, e11, e12),
            Vector3::new(e22, e21, e22),
        )
    }
}

#[cfg(feature = "impl")]
impl From<Matrix3> for na::Matrix3<f32> {
    fn from(v: Matrix3) -> Self {
        na::matrix![
            v.x.x, v.x.y, v.x.z;
            v.y.x, v.y.y, v.y.z;
            v.z.x, v.z.y, v.z.z
        ]
    }
}

#[cfg(feature = "impl")]
impl From<na::Matrix3<f32>> for Matrix3 {
    fn from(value: na::Matrix3<f32>) -> Self {
        let view = value.fixed_view::<3, 3>(0, 0);
        Self::new(
            Vector3::new(view[(0, 0)], view[(0, 1)], view[(0, 2)]),
            Vector3::new(view[(1, 0)], view[(1, 1)], view[(1, 2)]),
            Vector3::new(view[(2, 0)], view[(2, 1)], view[(2, 2)]),
        )
    }
}

#[cfg(feature = "impl")]
impl From<Matrix3> for na::Rotation3<f32> {
    fn from(value: Matrix3) -> Self {
        Self::from_matrix(&value.into())
    }
}

#[cfg(feature = "impl")]
impl From<na::Rotation3<f32>> for Matrix3 {
    fn from(value: na::Rotation3<f32>) -> Self {
        value.into_inner().into()
    }
}

#[cfg(feature = "impl")]
impl From<Matrix3> for na::UnitQuaternion<f32> {
    fn from(value: Matrix3) -> Self {
        Self::from_matrix(&value.into())
    }
}

#[cfg(feature = "impl")]
impl From<na::UnitQuaternion<f32>> for Matrix3 {
    fn from(value: na::UnitQuaternion<f32>) -> Self {
        value.to_rotation_matrix().into()
    }
}

/// Represents any color, including HDR colors.
///
/// ## See Also
/// * [`Color3uint8`](struct.Color3uint8.html), which is used instead of
///   `Color3` on some types and does not represent HDR colors.
/// * [Color3 on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/Color3)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color3 {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color3 {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    #[cfg(feature = "impl")]
    pub fn from_rgb(red: f32, green: f32, blue: f32) -> Self {
        Self::new(red / 255.0, green / 255.0, blue / 255.0)
    }

    #[cfg(feature = "impl")]
    pub fn from_hsv(hue: f32, saturation: f32, value: f32) -> Self {
        let c = Hsl::from(hue * 359.0, saturation * 100.0, value * 100.0).to_rgb();
        Self::new(c.get_red(), c.get_green(), c.get_blue())
    }

    #[cfg(feature = "impl")]
    pub fn from_hex(hex: &str) -> Option<Self> {
        let c = Rgb::from_hex_str(hex).ok()?;
        Some(Self::new(c.get_red(), c.get_green(), c.get_blue()))
    }

    #[cfg(feature = "impl")]
    pub fn to_hsv(&self) -> (f32, f32, f32) {
        let c = Rgb::from(self.r, self.g, self.b).to_hsl();
        (
            c.get_hue() / 359.0,
            c.get_saturation() / 100.0,
            c.get_lightness() / 100.0,
        )
    }

    #[cfg(feature = "impl")]
    pub fn to_hex(&self) -> String {
        Rgb::from(self.r, self.g, self.b).to_css_hex_string()
    }
}

#[cfg(feature = "mlua")]
impl<'lua> FromLua<'lua> for Color3 {
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        let LuaValue::UserData(value) = value else {
            return Err(LuaError::UserDataTypeMismatch);
        };
        if !value.is::<Self>() {
            return Err(LuaError::UserDataTypeMismatch);
        }
        Ok(Self::new(value.get("R")?, value.get("G")?, value.get("B")?))
    }
}

#[cfg(feature = "mlua")]
impl LuaUserData for Color3 {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("R", |_lua, this| Ok(this.r));
        fields.add_field_method_get("G", |_lua, this| Ok(this.g));
        fields.add_field_method_get("B", |_lua, this| Ok(this.b));
    }
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("Lerp", |_lua, this, (color, alpha): (Color3, f32)| {
            let beta = 1.0 - alpha;
            Ok(Self::new(
                this.r * alpha + color.r * beta,
                this.g * alpha + color.g * beta,
                this.b * alpha + color.b * beta,
            ))
        });
        methods.add_method("ToHSV", |_lua, this, ()| Ok(this.to_hsv()));
        methods.add_method("ToHex", |_lua, this, ()| Ok(this.to_hex()));
    }
}

impl From<Color3uint8> for Color3 {
    fn from(value: Color3uint8) -> Self {
        Self {
            r: value.r as f32 / 255.0,
            g: value.g as f32 / 255.0,
            b: value.b as f32 / 255.0,
        }
    }
}

/// Represents non-HDR colors, i.e. those whose individual color channels do not
/// exceed 1. This type is used for serializing properties like
/// [`BasePart.Color`][BasePart.Color], but is not exposed as a distinct type to
/// Lua code.
///
/// ## See Also
/// * [`Color3`](struct.Color3.html), which is more common and can represent HDR
///   colors.
///
/// [BasePart.Color]: https://developer.roblox.com/en-us/api-reference/property/BasePart/Color
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color3uint8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color3uint8 {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

impl From<Color3> for Color3uint8 {
    fn from(value: Color3) -> Self {
        Self {
            r: ((value.r.max(0.0).min(1.0)) * 255.0).round() as u8,
            g: ((value.g.max(0.0).min(1.0)) * 255.0).round() as u8,
            b: ((value.b.max(0.0).min(1.0)) * 255.0).round() as u8,
        }
    }
}

/// Represents a ray in 3D space. Direction does not have to be a unit vector,
/// and is used by APIs like [`Workspace:FindPartOnRay`][FindPartOnRay] to set a
/// max distance.
///
/// ## See Also
/// * [Ray on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/Ray)
///
/// [FindPartOnRay]: https://developer.roblox.com/en-us/api-reference/function/WorldRoot/FindPartOnRay
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        Self { origin, direction }
    }

    #[cfg(feature = "impl")]
    pub fn unit(&self) -> Self {
        Self::new(self.origin, self.direction.into_alg().normalize().into())
    }

    #[cfg(feature = "impl")]
    pub fn closest_point(&self, point: &Vector3) -> Vector3 {
        let t = self
            .direction
            .into_alg()
            .dot(&(point.into_alg() - self.origin.into_alg()));
        if t < 0.0 {
            self.origin
        } else {
            self.origin + self.direction * t
        }
    }

    #[cfg(feature = "impl")]
    pub fn distance(&self, point: &Vector3) -> f32 {
        (self.closest_point(point) - *point).magnitude()
    }
}

#[cfg(feature = "mlua")]
impl<'lua> FromLua<'lua> for Ray {
    fn from_lua(value: LuaValue<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        let LuaValue::UserData(value) = value else {
            return Err(LuaError::UserDataTypeMismatch);
        };
        if !value.is::<Self>() {
            return Err(LuaError::UserDataTypeMismatch);
        }
        Ok(Self::new(
            Vector3::from_lua(value.get("Origin")?, lua)?,
            Vector3::from_lua(value.get("Direction")?, lua)?,
        ))
    }
}

#[cfg(feature = "mlua")]
impl LuaUserData for Ray {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("Origin", |_lua, this| Ok(this.origin));
        fields.add_field_method_get("Direction", |_lua, this| Ok(this.direction));
        fields.add_field_method_get("Unit", |_lua, this| Ok(this.unit()));
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("ClosestPoint", |_lua, this, point: Vector3| {
            Ok(this.closest_point(&point))
        });
        methods.add_method("Distance", |_lua, this, point: Vector3| {
            Ok(this.distance(&point))
        });
    }
}

/// Represents a bounding box in 3D space.
///
/// ## See Also
/// * [`Region3int16`](struct.Region3int16.html)
/// * [Region3 on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/Region3)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Region3 {
    pub min: Vector3,
    pub max: Vector3,
}

impl Region3 {
    pub fn new(min: Vector3, max: Vector3) -> Self {
        Self { min, max }
    }

    #[cfg(feature = "impl")]
    pub fn expand_to_grid(&self, resolution: f32) -> Option<Self> {
        if !resolution.is_normal() {
            return None;
        }
        let min = self.min / resolution;
        let max = self.max / resolution;
        let min = Vector3::new(
            min.x.floor() * resolution,
            min.y.floor() * resolution,
            min.z.floor() * resolution,
        );
        let max = Vector3::new(
            max.x.floor() * resolution,
            max.y.floor() * resolution,
            max.z.floor() * resolution,
        );
        Some(Self::new(min, max))
    }
}

#[cfg(feature = "mlua")]
impl<'lua> FromLua<'lua> for Region3 {
    fn from_lua(value: LuaValue<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        let LuaValue::UserData(value) = value else {
            return Err(LuaError::UserDataTypeMismatch);
        };
        if !value.is::<Self>() {
            return Err(LuaError::UserDataTypeMismatch);
        }
        Ok(Self::new(
            Vector3::from_lua(value.get("Min")?, lua)?,
            Vector3::from_lua(value.get("Max")?, lua)?,
        ))
    }
}

#[cfg(feature = "mlua")]
impl LuaUserData for Region3 {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("Min", |_lua, this| Ok(this.min));
        fields.add_field_method_get("Max", |_lua, this| Ok(this.max));
    }
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("ExpandToGrid", |_lua, this, resolution: f32| {
            this.expand_to_grid(resolution)
                .ok_or_else(|| LuaError::external("Resolution has to be a positive number"))
        });
    }
}

/// A version of [`Region3`][Region3] that uses signed 16-bit integers instead
/// of floats. `Region3int16` is generally used in Terrain APIs.
///
/// ## See Also
/// * [`Region`][Region3]
/// * [Region3int16 on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/Region3int16)
///
/// [Region3]: struct.Region3.html
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Region3int16 {
    pub min: Vector3int16,
    pub max: Vector3int16,
}

impl Region3int16 {
    pub fn new(min: Vector3int16, max: Vector3int16) -> Self {
        Self { min, max }
    }
}

#[cfg(feature = "mlua")]
impl<'lua> FromLua<'lua> for Region3int16 {
    fn from_lua(value: LuaValue<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        let LuaValue::UserData(value) = value else {
            return Err(LuaError::UserDataTypeMismatch);
        };
        if !value.is::<Self>() {
            return Err(LuaError::UserDataTypeMismatch);
        }
        Ok(Self::new(
            Vector3int16::from_lua(value.get("Min")?, lua)?,
            Vector3int16::from_lua(value.get("Max")?, lua)?,
        ))
    }
}

#[cfg(feature = "mlua")]
impl LuaUserData for Region3int16 {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("Min", |_lua, this| Ok(this.min));
        fields.add_field_method_get("Max", |_lua, this| Ok(this.max));
    }
}

/// Represents a bounding rectangle in 2D space.
///
/// ## See Also
/// * [Rect on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/Rect)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub min: Vector2,
    pub max: Vector2,
}

impl Rect {
    pub fn new(min: Vector2, max: Vector2) -> Self {
        Self { min, max }
    }

    #[cfg(feature = "impl")]
    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }

    #[cfg(feature = "impl")]
    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }
}

#[cfg(feature = "mlua")]
impl<'lua> FromLua<'lua> for Rect {
    fn from_lua(value: LuaValue<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        let LuaValue::UserData(value) = value else {
            return Err(LuaError::UserDataTypeMismatch);
        };
        if !value.is::<Self>() {
            return Err(LuaError::UserDataTypeMismatch);
        }
        Ok(Self::new(
            Vector2::from_lua(value.get("Min")?, lua)?,
            Vector2::from_lua(value.get("Max")?, lua)?,
        ))
    }
}

#[cfg(feature = "mlua")]
impl LuaUserData for Rect {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("Min", |_lua, this| Ok(this.min));
        fields.add_field_method_get("Max", |_lua, this| Ok(this.max));
        fields.add_field_method_get("Height", |_lua, this| Ok(this.height()));
        fields.add_field_method_get("Width", |_lua, this| Ok(this.width()));
    }
}

/// Standard unit for measuring UI given as `scale`, a fraction of the
/// container's size and `offset`, display-indepdendent pixels.
///
/// ## See Also
/// * [UDim on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/UDim)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UDim {
    pub scale: f32,
    pub offset: i32,
}

impl UDim {
    pub fn new(scale: f32, offset: i32) -> Self {
        Self { scale, offset }
    }

    #[cfg(feature = "impl")]
    pub fn lerp(&self, goal: &UDim, alpha: f32) -> Self {
        let beta = 1.0 - alpha;
        let scale = self.scale * alpha + goal.scale * beta;
        let offset = self.offset as f32 * alpha + goal.offset as f32 * beta;
        Self::new(scale, offset.round() as i32)
    }
}

#[cfg(feature = "impl")]
impl Add for UDim {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.scale + rhs.scale, self.offset + rhs.offset)
    }
}

#[cfg(feature = "impl")]
impl Sub for UDim {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.scale - rhs.scale, self.offset - rhs.offset)
    }
}

#[cfg(feature = "impl")]
impl Neg for UDim {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.scale, -self.offset)
    }
}

#[cfg(feature = "mlua")]
impl<'lua> FromLua<'lua> for UDim {
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        let LuaValue::UserData(value) = value else {
            return Err(LuaError::UserDataTypeMismatch);
        };
        if !value.is::<Self>() {
            return Err(LuaError::UserDataTypeMismatch);
        }
        Ok(Self::new(value.get("Scale")?, value.get("Offset")?))
    }
}

#[cfg(feature = "mlua")]
impl LuaUserData for UDim {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("Scale", |_lua, this| Ok(this.scale));
        fields.add_field_method_get("Offset", |_lua, this| Ok(this.offset));
    }
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::Add, |_lua, &this, rhs: Self| Ok(this + rhs));
        methods.add_meta_method(LuaMetaMethod::Sub, |_lua, &this, rhs: Self| Ok(this - rhs));
        methods.add_meta_method(LuaMetaMethod::Unm, |_lua, &this, ()| Ok(-this));
    }
}

/// Standard 2D unit for measuring UI given as `scale`, a fraction of the
/// container's size and `offset`, display-indepdendent pixels.
///
/// ## See Also
/// * [UDim2 on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/UDim2)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UDim2 {
    pub x: UDim,
    pub y: UDim,
}

impl UDim2 {
    pub fn new(x: UDim, y: UDim) -> Self {
        Self { x, y }
    }

    #[cfg(feature = "impl")]
    pub fn lerp(&self, goal: &UDim2, alpha: f32) -> Self {
        Self::new(self.x.lerp(&goal.x, alpha), self.y.lerp(&goal.y, alpha))
    }
}

#[cfg(feature = "impl")]
impl Add for UDim2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

#[cfg(feature = "impl")]
impl Sub for UDim2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

#[cfg(feature = "impl")]
impl Neg for UDim2 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}

#[cfg(feature = "mlua")]
impl<'lua> FromLua<'lua> for UDim2 {
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        let LuaValue::UserData(value) = value else {
            return Err(LuaError::UserDataTypeMismatch);
        };
        if !value.is::<Self>() {
            return Err(LuaError::UserDataTypeMismatch);
        }
        Ok(Self::new(value.get("X")?, value.get("Y")?))
    }
}

#[cfg(feature = "mlua")]
impl LuaUserData for UDim2 {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("X", |_lua, this| Ok(this.x));
        fields.add_field_method_get("Y", |_lua, this| Ok(this.y));
        fields.add_field_method_get("Width", |_lua, this| Ok(this.x));
        fields.add_field_method_get("Height", |_lua, this| Ok(this.y));
    }
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::Add, |_lua, &this, rhs: Self| Ok(this + rhs));
        methods.add_meta_method(LuaMetaMethod::Sub, |_lua, &this, rhs: Self| Ok(this - rhs));
        methods.add_meta_method(LuaMetaMethod::Unm, |_lua, &this, ()| Ok(-this));
        methods.add_method("Lerp", |_lua, this, (goal, alpha): (UDim2, f32)| {
            Ok(this.lerp(&goal, alpha))
        })
    }
}

/// A range between two numbers.
///
/// ## See Also
/// * [NumberRange on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/NumberRange)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NumberRange {
    pub min: f32,
    pub max: f32,
}

impl NumberRange {
    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }
}

/// A series of colors that can be tweened through.
///
/// ## See Also
/// * [ColorSequence on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/ColorSequence)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct ColorSequence {
    pub keypoints: Vec<ColorSequenceKeypoint>,
}

/// A single color and point in time of a [`ColorSequence`][ColorSequence]
///
/// ## See Also
/// * [ColorSequenceKeypoint on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/ColorSequenceKeypoint)
///
/// [ColorSequence]: struct.ColorSequence.html
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct ColorSequenceKeypoint {
    pub time: f32,
    pub color: Color3,
}

impl ColorSequenceKeypoint {
    pub fn new(time: f32, color: Color3) -> Self {
        Self { time, color }
    }
}

/// A sequence of numbers on a timeline. Each point contains a timestamp, a
/// value, and a range that allows for randomized values.
///
/// ## See Also
/// * [NumberSequence on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/NumberSequence)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct NumberSequence {
    pub keypoints: Vec<NumberSequenceKeypoint>,
}

/// A single value, envelope, and point in time of a [`NumberSequence`][NumberSequence]
///
/// ## See Also
/// * [`NumberSequence`][NumberSequence]
/// * [NumberSequenceKeypoint on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/NumberSequenceKeypoint)
///
/// [NumberSequence]: struct.NumberSequence.html
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct NumberSequenceKeypoint {
    pub time: f32,
    pub value: f32,
    pub envelope: f32,
}

impl NumberSequenceKeypoint {
    pub fn new(time: f32, value: f32, envelope: f32) -> Self {
        Self {
            time,
            value,
            envelope,
        }
    }
}

#[cfg(feature = "serde")]
serde_tuple! {
    Vector2(x: f32, y: f32),
    Vector2int16(x: i16, y: i16),
    Vector3(x: f32, y: f32, z: f32),
    Vector3int16(x: i16, y: i16, z: i16),

    Color3(r: f32, g: f32, b: f32),
    Color3uint8(r: u8, g: u8, b: u8),

    UDim(scale: f32, offset: i32),
    UDim2(x: UDim, y: UDim),

    NumberRange(min: f32, max: f32),

    Rect(min: Vector2, max: Vector2),
    Region3(min: Vector3, max: Vector3),
    Region3int16(min: Vector3int16, max: Vector3int16),

    Matrix3(x: Vector3, y: Vector3, z: Vector3),
}

#[cfg(all(test, feature = "serde"))]
mod serde_test {
    use super::*;

    use std::fmt::Debug;

    use serde::{de::DeserializeOwned, Serialize};

    fn test_ser<T: Debug + PartialEq + Serialize + DeserializeOwned>(value: T, output: &str) {
        let serialized = serde_json::to_string(&value).unwrap();
        assert_eq!(serialized, output);

        let deserialized: T = serde_json::from_str(output).unwrap();
        assert_eq!(deserialized, value);
    }

    #[test]
    fn vec2_json() {
        test_ser(Vector2 { x: 2.0, y: 3.5 }, "[2.0,3.5]");
    }

    #[test]
    fn udim_json() {
        test_ser(
            UDim {
                scale: 1.0,
                offset: 175,
            },
            "[1.0,175]",
        );
    }

    #[test]
    fn udim2_json() {
        test_ser(
            UDim2 {
                x: UDim {
                    scale: 0.0,
                    offset: 30,
                },
                y: UDim {
                    scale: 1.0,
                    offset: 60,
                },
            },
            "[[0.0,30],[1.0,60]]",
        );
    }

    #[test]
    fn region3_json() {
        test_ser(
            Region3 {
                min: Vector3::new(-1.0, -2.0, -3.0),
                max: Vector3::new(4.0, 5.0, 6.0),
            },
            "[[-1.0,-2.0,-3.0],[4.0,5.0,6.0]]",
        );
    }

    #[test]
    fn matrix3_json() {
        test_ser(
            Matrix3 {
                x: Vector3::new(1.0, 2.0, 3.0),
                y: Vector3::new(4.0, 5.0, 6.0),
                z: Vector3::new(7.0, 8.0, 9.0),
            },
            "[[1.0,2.0,3.0],[4.0,5.0,6.0],[7.0,8.0,9.0]]",
        );
    }
}
