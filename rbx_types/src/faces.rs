use std::fmt;

use crate::lister::Lister;

#[cfg(feature = "mlua")]
use mlua::prelude::*;

bitflags::bitflags! {
    struct FaceFlags: u8 {
        const RIGHT = 1;
        const TOP = 2;
        const BACK = 4;
        const LEFT = 8;
        const BOTTOM = 16;
        const FRONT = 32;
    }
}

/// Represents a set of zero or more faces of a cube.
///
/// ## See Also
/// * [Faces on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/Faces)
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Faces {
    flags: FaceFlags,
}

impl Faces {
    pub const RIGHT: Self = Self {
        flags: FaceFlags::RIGHT,
    };

    pub const TOP: Self = Self {
        flags: FaceFlags::TOP,
    };

    pub const BACK: Self = Self {
        flags: FaceFlags::BACK,
    };

    pub const LEFT: Self = Self {
        flags: FaceFlags::LEFT,
    };

    pub const BOTTOM: Self = Self {
        flags: FaceFlags::BOTTOM,
    };

    pub const FRONT: Self = Self {
        flags: FaceFlags::FRONT,
    };
}

impl Faces {
    pub fn new(right: bool, top: bool, back: bool, left: bool, bottom: bool, front: bool) -> Self {
        macro_rules! flag_if {
            ($cond:ident, $flag_name:ident) => {
                if $cond {
                    FaceFlags::$flag_name
                } else {
                    FaceFlags::empty()
                }
            };
        }

        Self {
            flags: flag_if!(right, RIGHT)
                | flag_if!(top, TOP)
                | flag_if!(back, BACK)
                | flag_if!(left, LEFT)
                | flag_if!(bottom, BOTTOM)
                | flag_if!(front, FRONT),
        }
    }
    pub const fn empty() -> Self {
        Self {
            flags: FaceFlags::empty(),
        }
    }

    pub const fn all() -> Self {
        Self {
            flags: FaceFlags::all(),
        }
    }

    pub const fn contains(self, other: Self) -> bool {
        self.flags.contains(other.flags)
    }

    pub const fn bits(self) -> u8 {
        self.flags.bits()
    }

    pub fn from_bits(bits: u8) -> Option<Self> {
        FaceFlags::from_bits(bits).map(|flags| Self { flags })
    }

    #[cfg(feature = "serde")]
    fn len(self) -> usize {
        self.bits().count_ones() as usize
    }
}

#[cfg(feature = "mlua")]
impl<'lua> FromLua<'lua> for Faces {
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        let LuaValue::UserData(value) = value else {
            return Err(LuaError::UserDataTypeMismatch);
        };
        if !value.is::<Self>() {
            return Err(LuaError::UserDataTypeMismatch);
        }
        Ok(Self::new(
            value.get("Right")?,
            value.get("Top")?,
            value.get("Back")?,
            value.get("Left")?,
            value.get("Bottom")?,
            value.get("Front")?,
        ))
    }
}

#[cfg(feature = "mlua")]
impl LuaUserData for Faces {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        macro_rules! impl_flag {
            ($field_name:literal, $flag_name:ident) => {
                fields.add_field_method_get($field_name, |_lua, this| {
                    Ok(this.flags.contains(FaceFlags::$flag_name))
                });
                fields.add_field_method_set($field_name, |_lua, this, val: bool| {
                    Ok(this.flags.set(FaceFlags::$flag_name, val))
                })
            };
        }
        impl_flag!("Right", RIGHT);
        impl_flag!("Top", TOP);
        impl_flag!("Back", BACK);
        impl_flag!("Left", LEFT);
        impl_flag!("Bottom", BOTTOM);
        impl_flag!("Front", FRONT);
    }
}

impl fmt::Debug for Faces {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        let mut list = Lister::new();

        write!(out, "Faces(")?;

        if self.contains(Faces::RIGHT) {
            list.write(out, "Right")?;
        }

        if self.contains(Faces::TOP) {
            list.write(out, "Top")?;
        }

        if self.contains(Faces::BACK) {
            list.write(out, "Back")?;
        }

        if self.contains(Faces::LEFT) {
            list.write(out, "Left")?;
        }

        if self.contains(Faces::BOTTOM) {
            list.write(out, "Bottom")?;
        }

        if self.contains(Faces::FRONT) {
            list.write(out, "Front")?;
        }

        write!(out, ")")
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use super::*;

    use std::fmt;

    use serde::{
        de::{Error as _, SeqAccess, Visitor},
        ser::SerializeSeq,
        Deserialize, Deserializer, Serialize, Serializer,
    };

    impl Serialize for Faces {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            if serializer.is_human_readable() {
                let mut seq = serializer.serialize_seq(Some(self.len()))?;

                if self.contains(Self::RIGHT) {
                    seq.serialize_element("Right")?;
                }

                if self.contains(Self::TOP) {
                    seq.serialize_element("Top")?;
                }

                if self.contains(Self::BACK) {
                    seq.serialize_element("Back")?;
                }

                if self.contains(Self::LEFT) {
                    seq.serialize_element("Left")?;
                }

                if self.contains(Self::BOTTOM) {
                    seq.serialize_element("Bottom")?;
                }

                if self.contains(Self::FRONT) {
                    seq.serialize_element("Front")?;
                }

                seq.end()
            } else {
                serializer.serialize_u8(self.bits())
            }
        }
    }

    struct HumanVisitor;

    impl<'de> Visitor<'de> for HumanVisitor {
        type Value = Faces;

        fn expecting(&self, out: &mut fmt::Formatter) -> fmt::Result {
            write!(out, "a list of strings representing faces")
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let mut flags = FaceFlags::empty();

            while let Some(face_str) = seq.next_element::<&str>()? {
                match face_str {
                    "Right" => flags |= FaceFlags::RIGHT,
                    "Top" => flags |= FaceFlags::TOP,
                    "Back" => flags |= FaceFlags::BACK,
                    "Left" => flags |= FaceFlags::LEFT,
                    "Bottom" => flags |= FaceFlags::BOTTOM,
                    "Front" => flags |= FaceFlags::FRONT,
                    _ => {
                        return Err(A::Error::custom(format!("invalid face '{}'", face_str)));
                    }
                }
            }

            Ok(Faces { flags })
        }
    }

    impl<'de> Deserialize<'de> for Faces {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            if deserializer.is_human_readable() {
                deserializer.deserialize_seq(HumanVisitor)
            } else {
                let value = u8::deserialize(deserializer)?;

                Faces::from_bits(value)
                    .ok_or_else(|| D::Error::custom("value must a u8 bitmask of faces"))
            }
        }
    }
}

#[cfg(all(test, feature = "serde"))]
mod serde_test {
    use super::*;

    #[test]
    fn human_de() {
        let empty: Faces = serde_json::from_str("[]").unwrap();
        assert_eq!(empty, Faces::empty());

        let x: Faces = serde_json::from_str(r#"["Right"]"#).unwrap();
        assert_eq!(x, Faces::RIGHT);

        let all: Faces =
            serde_json::from_str(r#"["Right", "Top", "Back", "Left", "Bottom", "Front"]"#).unwrap();
        assert_eq!(all, Faces::all());
    }

    #[test]
    fn human_ser() {
        let empty = serde_json::to_string(&Faces::empty()).unwrap();
        assert_eq!(empty, "[]");

        let x = serde_json::to_string(&Faces::LEFT).unwrap();
        assert_eq!(x, r#"["Left"]"#);

        let all = serde_json::to_string(&Faces::all()).unwrap();
        assert_eq!(all, r#"["Right","Top","Back","Left","Bottom","Front"]"#);
    }

    #[test]
    fn human_duplicate() {
        let x: Faces = serde_json::from_str(r#"["Right", "Right", "Right", "Right"]"#).unwrap();
        assert_eq!(x, Faces::RIGHT);
    }

    #[test]
    fn human_invalid() {
        // calzone is not a face of a cube
        let invalid = serde_json::from_str::<Faces>(r#"["calzone"]"#);
        assert!(invalid.is_err());
    }

    #[test]
    fn non_human() {
        let empty = Faces::empty();
        let ser_empty = bincode::serialize(&empty).unwrap();
        let de_empty = bincode::deserialize(&ser_empty).unwrap();
        assert_eq!(empty, de_empty);

        let right = Faces::RIGHT;
        let ser_right = bincode::serialize(&right).unwrap();
        let de_right = bincode::deserialize(&ser_right).unwrap();
        assert_eq!(right, de_right);

        let all = Faces::all();
        let ser_all = bincode::serialize(&all).unwrap();
        let de_all = bincode::deserialize(&ser_all).unwrap();
        assert_eq!(all, de_all);
    }
}
