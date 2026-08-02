use std::str::FromStr;

use crate::{Decimal, obj_money::ObjMoney};
use ::serde::{Deserialize, Deserializer, Serialize, Serializer};

impl<const IS_RAW: bool> serde::Serialize for ObjMoney<IS_RAW> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let amount = serde_json::Number::from_str(&self.amount().to_string())
            .map_err(|_| ::serde::ser::Error::custom("cannot convert Decimal to JSON Number"))?;

        let mut state = serializer.serialize_struct("obj_money", 2)?;
        state.serialize_field("code", self.code())?;
        state.serialize_field("amount", &amount)?;
        state.end()
    }
}

impl<'de, const IS_RAW: bool> serde::Deserialize<'de> for ObjMoney<IS_RAW> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::Deserialize;
        use std::str::FromStr;

        #[derive(Debug, Deserialize)]
        struct ObjMoneySerde {
            code: String,
            amount: serde_json::Number,
        }

        let value = ObjMoneySerde::deserialize(deserializer)?;

        let amount =
            Decimal::from_str(&value.amount.to_string()).map_err(serde::de::Error::custom)?;

        ObjMoney::try_new(&value.code, amount).map_err(serde::de::Error::custom)
    }
}

// serde string
//
//

pub mod str_code_comma {
    use super::*;

    pub fn serialize<S, const IS_RAW: bool>(
        value: &ObjMoney<IS_RAW>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        value
            .format(crate::fmt::CODE_FORMAT, ",", ".")
            .serialize(serializer)
    }

    pub fn deserialize<'de, D, const IS_RAW: bool>(
        deserializer: D,
    ) -> Result<ObjMoney<IS_RAW>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ObjMoney::from_str_code(&s, ",", ".").map_err(::serde::de::Error::custom)
    }
}

pub mod str_code_dot {
    use super::*;

    pub fn serialize<S, const IS_RAW: bool>(
        value: &ObjMoney<IS_RAW>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        value
            .format(crate::fmt::CODE_FORMAT, ".", ",")
            .serialize(serializer)
    }

    pub fn deserialize<'de, D, const IS_RAW: bool>(
        deserializer: D,
    ) -> Result<ObjMoney<IS_RAW>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ObjMoney::from_str_code(&s, ".", ",").map_err(::serde::de::Error::custom)
    }
}
