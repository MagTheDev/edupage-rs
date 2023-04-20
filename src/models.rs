use serde::{Serialize, Deserialize};
use crate::parsers::*;

/// A struct representing a single teacher  
/// 
/// ```rust
/// // Sample Data:
/// "59276": { 
///      "id": "59276",
///      "firstname": "First",
///      "lastname": "Second",
///      "short": "",
///      "gender": "M",
///      "classroomid": "",
///      "datefrom": "",
///      "dateto": "",
///      "cb_hidden": 0,
///      "isOut": false
/// }
/// ```

#[derive(Debug, Serialize, Deserialize)]
pub struct Teacher {

    #[serde(with = "option_i64_id")]
    pub id: Option<i64>,

    #[serde(rename = "firstname")]
    pub first_name: String,

    #[serde(rename = "lastname")]
    pub last_name: String,

    #[serde(deserialize_with = "crate::parsers::short::deserialize")]
    pub short: Option<String>,

    #[serde(with = "gender")]
    pub gender: Gender,

    #[serde(rename = "classroomid")]
    pub classroom_id: String,

    #[serde(rename = "datefrom")]
    pub date_from: String,

    #[serde(rename = "dateto")]
    pub date_to: String,

    pub cb_hidden: i32,

    #[serde(rename = "isOut")]
    pub is_out: bool

}

#[derive(Debug)]
pub enum Gender {
    Male,
    Female,
    Unknown
}