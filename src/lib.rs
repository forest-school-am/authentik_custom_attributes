use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub type RawAttrs = HashMap<String, Value>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoginAccount {
    pub kind: String,
    pub address: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ForestSchoolGroup {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub leaders: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub manager: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub google_sync: Option<GoogleSync>,
    #[serde(flatten)]
    pub other: HashMap<String, Value>,
}

/// forest_school sub-object as it appears on user attributes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ForestSchoolUser {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub logins: Vec<LoginAccount>,
    #[serde(default, rename = "user-defined", skip_serializing_if = "HashMap::is_empty")]
    pub user_defined: HashMap<String, String>,
    #[serde(flatten)]
    pub other: HashMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GoogleSync {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recursive_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direct_name: Option<String>,
    #[serde(flatten)]
    pub other: HashMap<String, Value>,
}

// ---------------------------------------------------------------------------
// Top-level attribute maps
// ---------------------------------------------------------------------------

/// Full attribute map stored on an Authentik group.
/// Known namespaces are typed; anything else is preserved in `other`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GroupAttributes {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forest_school: Option<ForestSchoolGroup>,
    #[serde(flatten)]
    pub other: HashMap<String, Value>,
}

/// Full attribute map stored on an Authentik user.
/// Known namespaces are typed; anything else is preserved in `other`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAttributes {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forest_school: Option<ForestSchoolUser>,
    #[serde(flatten)]
    pub other: HashMap<String, Value>,
}

impl GroupAttributes {
    pub fn from_raw(raw: Option<RawAttrs>) -> Result<Self, serde_json::Error> {
        match raw {
            None => Ok(Self::default()),
            Some(map) => serde_json::from_value(Value::Object(map.into_iter().collect())),
        }
    }

    pub fn into_raw(self) -> Result<RawAttrs, serde_json::Error> {
        match serde_json::to_value(self)? {
            Value::Object(map) => Ok(map.into_iter().collect()),
            _ => Ok(HashMap::new()),
        }
    }
}

impl UserAttributes {
    pub fn from_raw(raw: Option<RawAttrs>) -> Result<Self, serde_json::Error> {
        match raw {
            None => Ok(Self::default()),
            Some(map) => serde_json::from_value(Value::Object(map.into_iter().collect())),
        }
    }

    pub fn into_raw(self) -> Result<RawAttrs, serde_json::Error> {
        match serde_json::to_value(self)? {
            Value::Object(map) => Ok(map.into_iter().collect()),
            _ => Ok(HashMap::new()),
        }
    }
}
