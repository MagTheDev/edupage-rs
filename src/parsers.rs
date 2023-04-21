pub mod login {
    use serde_json::Value;

    pub fn get_json(data: String) -> Value {
        let json_string = data
            .split("userhome(")
            .nth(1)
            .unwrap()
            .split(");")
            .next()
            .unwrap()
            .replace(['\t', '\n', '\r'], "");

        #[cfg(feature = "dump")]
        {
            use std::{fs::OpenOptions, io::Write};
            let mut f = OpenOptions::new()
                .write(true)
                .create(true)
                .open("data.dump.json")
                .unwrap();
            f.write_all(json_string.as_bytes()).unwrap();
        }

        serde_json::from_str(&json_string).unwrap()
    }
}

pub mod gender {

    use crate::models::Gender;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(gender: &Gender, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match gender {
            Gender::Male => "M",
            Gender::Female => "F",
            Gender::Unknown => "",
        })
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Gender, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = &String::deserialize(deserializer)?.to_lowercase();
        match s {
            "m" => Ok(Gender::Male),
            "f" => Ok(Gender::Female),
            "" => Ok(Gender::Unknown),
            _ => Err(serde::de::Error::custom(format!(
                "Failed deserializing gender: {}",
                s
            ))),
        }
    }
}

pub mod short {
    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?.to_lowercase();
        if s.is_empty() {
            Ok(None)
        } else {
            Ok(Some(s))
        }
    }
}

pub mod option_string_i64 {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(id: &Option<i64>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if id.is_none() {
            serializer.serialize_none()
        } else {
            serializer.serialize_i64(id.unwrap())
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<String> = Deserialize::deserialize(deserializer)?;

        if s.is_none() {
            return Ok(None);
        }

        let s = &s.unwrap();
        if s.is_empty() {
            return Ok(None);
        }

        match s.parse() {
            Ok(i) => Ok(Some(i)),
            Err(_) => Ok(None),
        }
    }
}

pub mod string_i64 {

    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(id: &i64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(id.to_owned())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<i64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;

        match s.parse() {
            Ok(i) => Ok(i),
            Err(_) => {
                unreachable!("This code should be unreachable as id should never be non numerical")
            }
        }
    }
}
