use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use std::fmt;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
pub enum UPos {
    Pixels(f64), // TO BE IMPLEMENTED lmao
    Points(f64),
}

pub struct UPosVisitor;

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct UVector2(pub UPos, pub UPos);

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct UVector4(pub UPos, pub UPos, pub UPos, pub UPos);

impl UPos {
    pub fn get(&self) -> f64 {
        use UPos::*;
        match self {
            Pixels(_m) => todo!(),
            Points(m) => {
                if m < &0f64 {
                    panic!("UPos cannot be less than 0")
                }
                m.clone()
            }
        }
    }
}

impl Serialize for UPos {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use UPos::*;
        match self {
            Pixels(p) => serializer.serialize_str(&format!("{}px", p)),
            Points(p) => serializer.serialize_str(&format!("{}pt", p)),
        }
    }
}

impl<'de> Deserialize<'de> for UPos {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(UPosVisitor)
    }
}

impl<'de> Visitor<'de> for UPosVisitor {
    type Value = UPos;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a positive 64-bit floating point number with pixel or point suffix")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> {
        let (value, suffix) = value.split_at(value.len() - 2);
        let float_value = f64::from_str(value).expect("Float should be valid");
        Ok(match suffix {
            "px" => UPos::Pixels(float_value),
            "pt" => UPos::Points(float_value),
            e => panic!("Suffix on Pos value should be `px` or `pt`, not `{}`", e),
        })
    }
}
