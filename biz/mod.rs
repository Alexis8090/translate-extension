use std::clone;
use std::fmt::Debug;
use std::{fmt::Display, str::FromStr};

use rusqlite_from_row::FromRow;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
// use sqlx::types::time::Date;
// use sqlx::FromRow;
// use time::OffsetDateTime;
use validator::Validate;
use derivative::Derivative;


mod string_to_number {

    use core::num;
    use std::{fmt::{Debug, Display}, str::FromStr};

    use serde::{self, de, Deserialize, Deserializer, Serialize, Serializer};
    use serde_json::{json, Value};

    use crate::svr::error::HttpResError;

    pub fn serialize<T:  Serialize + Display, S>(number: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
         match number {
            Some(v) => serializer.serialize_some(v),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        D: Deserializer<'de> ,
        T: FromStr + Deserialize<'de>,      // 添加 FromStr 约束
        T::Err: Display, // 确保解析错误可以使用 Display 格式化
    {
        let json_value = Value::deserialize(deserializer)?;
        match json_value {
            Value::Null => Ok(None),
            Value::Bool(_) => {
                Err(de::Error::custom("boolean values are not supported"))
            },
            Value::String(string) => {
                 if string.is_empty() {
                    Ok(None)
                } else {
                    string.parse::<T>().map(Some).map_err(de::Error::custom)
                }
            },
            Value::Number(number) =>  T::deserialize(number).map(Some).map_err(de::Error::custom),
            Value::Array(_) =>   Err(de::Error::custom("arrays are not supported")),
            Value::Object(_) =>    Err(de::Error::custom("objects are not supported")),
        }
    }
}



fn default_0_u16() -> Option<u16> { Some(0) }
fn default_10_u8() -> Option<u8> { Some(10) }
#[derive(Deserialize,Validate,Debug,Serialize,Clone)]
pub struct PaginatorWith<T> {//P, PS,
    #[serde(default = "default_0_u16", with = "string_to_number")] // 解决query字符串要转数字 其中包括None
    pub pn: Option<u16> ,

    #[serde(default = "default_10_u8", with = "string_to_number")] // 解决query字符串要转数字 其中包括None
    pub ps: Option<u8>,


    #[serde(default, with = "string_to_number")] // 解决query字符串要转数字 其中包括None
    pub id: Option<i64>,

    #[serde(flatten)]
    pub custom: T,
}



#[derive(Deserialize,Serialize,FromRow,Validate,Derivative)] // 1. 定义请求体结构体
#[derivative(Debug)]
pub struct Table<T> {
    pub id:i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "time::serde::rfc3339::option")]
    #[serde(skip_deserializing)]
    #[derivative(Debug="ignore")]
    pub created_at:Option<OffsetDateTime>,


    // 可以插入数据库 就是可以De
    // 不可以反悔给钱对，就是不序列化
    #[from_row(skip)]
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    #[derivative(Debug="ignore")]
    pub updated_at:Option<OffsetDateTime>,

    #[from_row(skip)]
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    #[derivative(Debug="ignore")]
    pub deleted_at:Option<OffsetDateTime>,

    #[serde(flatten)]
    #[from_row(flatten)]
    pub custom:T
}

// ============================ 杂货

#[derive(Deserialize,Serialize,Validate,FromRow,Debug,Clone)] // 1. 定义请求体结构体
pub struct UserPayload {
    #[validate(length(
        min = 3,
        max = 20,
        message = "Username must be between 3 and 20 characters"
    ))]
    pub name: Option<String>,

    #[serde(default, with = "string_to_number")] // 解决query字符串要转数字 其中包括None,这个defualt解决query 不传 和 &age= 的区别
    pub age: Option<u8>,
}
