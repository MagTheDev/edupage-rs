use crate::parsers::*;
use serde::{Deserialize, Serialize};

/// A struct representing a single teacher  
///
/// ```text
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
    #[serde(with = "option_string_i64")]
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
    pub is_out: bool,
}

/// A struct representing a single student
/// ```text
/// // Sample Data:
///    "43257": {
///        "id": "43257",
///        "classid": "43200",
///        "firstname": "Name",
///        "lastname": "Surname",
///        "parent1id": "-1098",
///        "parent2id": "-1573",
///        "parent3id": "", // ??
///        "gender": "M",
///        "datefrom": "2017-09-01",
///        "dateto": "",
///        "numberinclass": "30",
///        "number": "",
///        "odborid": 33992,
///        "zus_rcs_short": "",
///        "zus_rcs_note": "",
///        "isOut": false
///    }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Student {
    #[serde(with = "string_i64")]
    pub id: i64,
    #[serde(rename = "classid", with = "string_i64")]
    pub class_id: i64,
    #[serde(rename = "firstname")]
    pub first_name: String,
    #[serde(rename = "lastname")]
    pub last_name: String,
    #[serde(rename = "parent1id", with = "string_i64")]
    pub first_parent_id: i64,
    #[serde(rename = "parent2id", with = "option_string_i64")]
    pub second_parent_id: Option<i64>,
    #[serde(rename = "parent3id", with = "option_string_i64")]
    pub third_parent_id: Option<i64>, // ??
    #[serde(with = "gender")]
    pub gender: Gender,
    #[serde(rename = "datefrom")]
    pub date_from: Option<String>,
    #[serde(rename = "dateto")]
    pub date_to: Option<String>,
    #[serde(rename = "numberinclass", with = "option_string_i64")]
    pub number_in_class: Option<i64>,
    #[serde(rename = "isOut")]
    pub is_out: bool
}

#[derive(Debug, Clone)]
pub enum Gender {
    Male,
    Female,
    Unknown,
}
