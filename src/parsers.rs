pub const TIMELINE_ITEM_TYPE_NAMES: [&'static str; 19] = [
    "news",
    "sprava",
    "h_dailyplan",
    "student_absent",
    "confirmation",
    "h_clearplany",
    "h_financie",
    "h_stravamenu",
    "h_clearisicdata",
    "substitution",
    "h_clearcache",
    "event",
    "h_homework",
    "znamka",
    "h_substitution",
    "h_znamky",
    "homework",
    "h_cleardbi",
    "testpridelenie",
];

pub mod timeline_item_type {
    use num_enum::TryFromPrimitiveError;
    use serde::{Serializer, Deserializer, Deserialize};
    use crate::models::TimelineItemType;
    use super::TIMELINE_ITEM_TYPE_NAMES;


    impl TryFrom<&str> for TimelineItemType {
        type Error = TryFromPrimitiveError<TimelineItemType>;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            
            for (index, item) in TIMELINE_ITEM_TYPE_NAMES.iter().enumerate() {
                if *item == value {
                    return index.try_into()
                }
            }
            Ok(TimelineItemType::Unknown)
        }
    }

    impl TimelineItemType {

        pub fn as_str(&self) -> &str {
            let integer_val = *self as usize;
            return self.key_name_for_n(integer_val);
        }
        pub fn key_name_for_n(&self, n: usize) -> &'static str {
            TIMELINE_ITEM_TYPE_NAMES[n]
        }

    }

    pub fn serialize<S>(item_type: &TimelineItemType, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(item_type.as_str())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<TimelineItemType, D::Error> where D: Deserializer<'de> {
        let str: &str = &String::deserialize(deserializer)?;
        let item_type: TimelineItemType = match str.try_into() {
            Ok(x) => x,
            Err(_) => {
                return Err(serde::de::Error::custom("Failed deserializing TimelineItemType"));
            },
        };
        Ok(item_type)
    }

}

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
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

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

    impl Serialize for Gender {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_str(match self {
                Gender::Male => "M",
                Gender::Female => "F",
                Gender::Unknown => "",
            })
        }
    }

    impl<'de> Deserialize<'de> for Gender {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s: &str = &String::deserialize(deserializer)?.to_lowercase();
            match s {
                "m" => Ok(Gender::Male),
                "f" => Ok(Gender::Female),
                "" => Ok(Gender::Unknown),
                _ => Err(serde::de::Error::custom(format!(
                    "Failed deserializing gender: '{}'",
                    s
                ))),
            }
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

pub mod user_id {
    use serde::{Serialize, Deserialize, Deserializer};

    use crate::models::Id;


    pub fn to_string(item_type: &Id) -> String {
        match item_type {
            Id::Teacher(id) => format!("Ucitel{}", id),
            Id::Student(id) => format!("Student{}", id),
            Id::Parent(id) => format!("Rodic{}", id),
            Id::Class(id) => format!("Trieda{}", id),
            Id::Plan(id) => format!("Plan{}", id),
            Id::CustomPlan(id) => format!("CustPlan{}", id),
            Id::StudentClass(id) => format!("StudTrieda{}", id),
            Id::OnlyStudent(id) => format!("StudentOnly{}", id),
            Id::StudentPlan(id) => format!("StudPlan{}", id),
            Id::OnlyAllStudents => "StudentOnly*".to_string(),
            Id::AllStudents => "Student*".to_string(),
            Id::AllTeachers => "Ucitel*".to_string(),
            Id::Everyone => "*".to_string(),
        }
    }


    pub fn parse_user_id(uid: &str) -> Option<Id> {
        
        let user_type = match uid {
            "*" => Some(Id::Everyone),
            "Student*" => Some(Id::AllStudents),
            "Teacher*" => Some(Id::AllTeachers),
            "StudentOnly*" => Some(Id::OnlyAllStudents),
            _ => None
        };
        
        if user_type.is_some() {
            return user_type;
        }
        
        let mut id = String::new();
        let mut user_type = String::new();

        for char in uid.chars() {
            if char.is_alphabetic() {
                user_type +=  &char.to_string();
            } else {
                id += &char.to_string();
            }
        }

        let id = id.parse().unwrap();
        let user_type: &str = &user_type;

        Some(match user_type {
            "Ucitel" => Id::Teacher(id),
            "Student" => Id::Student(id),
            "Rodic" => Id::Parent(id),
            "Trieda" => Id::Class(id),
            "Plan" => Id::Plan(id),
            "CustPlan" => Id::CustomPlan(id),
            "StudTrieda" => Id::StudentClass(id),
            "StudentOnly" => Id::OnlyStudent(id),
            "StudPlan" => Id::StudentPlan(id),
            _ => return None,
        })

    }

    impl Serialize for Id {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let string_representation = to_string(self);
            serializer.serialize_str(&string_representation)
        }
    }
    
    impl<'de> Deserialize<'de> for Id {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s: &str = &String::deserialize(deserializer)?;
    
            let user_id = parse_user_id(s);
            if user_id.is_none() {
                return Err(serde::de::Error::custom(format!("Unexpected user type")));
            }
    
            return Ok(user_id.unwrap());
        }
    }
    

}

