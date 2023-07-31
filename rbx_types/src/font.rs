#[cfg(feature = "mlua")]
use mlua::prelude::*;

#[cfg(feature = "mlua")]
use crate::content::Content;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FontWeight {
    Thin,
    ExtraLight,
    Light,
    #[default]
    Regular,
    Medium,
    SemiBold,
    Bold,
    ExtraBold,
    Heavy,
}

impl FontWeight {
    pub fn from_u16(weight: u16) -> Option<Self> {
        Some(match weight {
            100 => FontWeight::Thin,
            200 => FontWeight::ExtraLight,
            300 => FontWeight::Light,
            400 => FontWeight::Regular,
            500 => FontWeight::Medium,
            600 => FontWeight::SemiBold,
            700 => FontWeight::Bold,
            800 => FontWeight::ExtraBold,
            900 => FontWeight::Heavy,
            _ => return None,
        })
    }
    pub fn as_u16(self) -> u16 {
        match self {
            FontWeight::Thin => 100,
            FontWeight::ExtraLight => 200,
            FontWeight::Light => 300,
            FontWeight::Regular => 400,
            FontWeight::Medium => 500,
            FontWeight::SemiBold => 600,
            FontWeight::Bold => 700,
            FontWeight::ExtraBold => 800,
            FontWeight::Heavy => 900,
        }
    }
}

#[cfg(feature = "mlua")]
impl<'lua> IntoLua<'lua> for FontWeight {
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        self.as_u16().into_lua(lua)
    }
}

#[cfg(feature = "mlua")]
impl<'lua> FromLua<'lua> for FontWeight {
    fn from_lua(value: LuaValue<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        Self::from_u16(u16::from_lua(value, lua)?).ok_or_else(|| LuaError::UserDataTypeMismatch)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FontStyle {
    #[default]
    Normal,
    Italic,
}

impl FontStyle {
    pub fn from_u8(style: u8) -> Option<Self> {
        Some(match style {
            0 => FontStyle::Normal,
            1 => FontStyle::Italic,
            _ => return None,
        })
    }

    pub fn as_u8(self) -> u8 {
        match self {
            FontStyle::Normal => 0,
            FontStyle::Italic => 1,
        }
    }
}

#[cfg(feature = "mlua")]
impl<'lua> IntoLua<'lua> for FontStyle {
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        self.as_u8().into_lua(lua)
    }
}

#[cfg(feature = "mlua")]
impl<'lua> FromLua<'lua> for FontStyle {
    fn from_lua(value: LuaValue<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        Self::from_u8(u8::from_lua(value, lua)?).ok_or_else(|| LuaError::UserDataTypeMismatch)
    }
}

/// A font face consisting of a typeface and other style properties.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct Font {
    pub family: String,
    pub weight: FontWeight,
    pub style: FontStyle,
    pub cached_face_id: Option<String>,
}

impl Default for Font {
    fn default() -> Self {
        Self {
            family: "rbxasset://fonts/families/SourceSansPro.json".to_owned(),
            weight: FontWeight::default(),
            style: FontStyle::default(),
            cached_face_id: None,
        }
    }
}

impl Font {
    pub fn new(family: &str, weight: FontWeight, style: FontStyle) -> Self {
        Self {
            family: family.to_owned(),
            weight,
            style,
            cached_face_id: None,
        }
    }
    pub fn regular(family: &str) -> Self {
        Self {
            family: family.to_owned(),
            ..Default::default()
        }
    }
}

#[cfg(feature = "mlua")]
impl<'lua> FromLua<'lua> for Font {
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        let LuaValue::UserData(value) = value else {
            return Err(LuaError::UserDataTypeMismatch);
        };
        if !value.is::<Self>() {
            return Err(LuaError::UserDataTypeMismatch);
        }
        Ok(Self::new(
            &value.get::<_, String>("Family")?,
            value.get("Weight")?,
            value.get("Style")?,
        ))
    }
}

#[cfg(feature = "mlua")]
impl LuaUserData for Font {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("Family", |_lua, this| {
            Ok(Content::from(&this.family as &str))
        });
        fields.add_field_method_get("Weight", |_lua, this| Ok(this.weight));
        fields.add_field_method_get("Style", |_lua, this| Ok(this.style));
        fields.add_field_method_get("Bold", |_lua, this| {
            Ok(match this.weight {
                FontWeight::SemiBold | FontWeight::Bold | FontWeight::ExtraBold => true,
                _ => false,
            })
        });
    }
}
