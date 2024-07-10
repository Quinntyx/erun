use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use std::fmt;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
pub enum Pos {
    Pixels(f64), // TO BE IMPLEMENTED lmao
    Points(f64),
}

struct PosVisitor;

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct Vector2(pub Pos, pub Pos);

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct Vector4(pub Pos, pub Pos, pub Pos, pub Pos);

impl Pos {}

impl Serialize for Pos {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use Pos::*;
        match self {
            Pixels(p) => serializer.serialize_str(&format!("{}px", p)),
            Points(p) => serializer.serialize_str(&format!("{}pt", p)),
        }
    }
}

impl<'de> Deserialize<'de> for Pos {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(PosVisitor)
    }
}

impl<'de> Visitor<'de> for PosVisitor {
    type Value = Pos;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a 64-bit floating point number with pixel or point suffix")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> {
        let (value, suffix) = value.split_at(value.len() - 2);
        let float_value = f64::from_str(value).expect("Float should be valid");
        Ok(match suffix {
            "px" => Pos::Pixels(float_value),
            "pt" => Pos::Points(float_value),
            e => panic!("Suffix on Pos value should be `px` or `pt`, not `{}`", e),
        })
    }
}
