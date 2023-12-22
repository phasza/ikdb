use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug)]
enum CommandStatus {
    Success,
    Failure,
}

impl Serialize for CommandStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let variant_str = match self {
            CommandStatus::Success => "success",
            CommandStatus::Failure => "failure",
        };

        serializer.serialize_str(variant_str)
    }
}

impl<'de> Deserialize<'de> for CommandStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let variant_str = String::deserialize(deserializer)?;

        match variant_str.as_str() {
            "success" => Ok(CommandStatus::Success),
            "failure" => Ok(CommandStatus::Failure),
            _ => Err(serde::de::Error::unknown_variant(
                &variant_str,
                &["success", "failure"],
            )),
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct TransformCommandResponse {
    status: CommandStatus,
    num_rows: i32,
    error: Vec<String>,
    warning: Vec<String>,
}

impl TransformCommandResponse {
    // Constructor for success
    pub fn success(num_rows: i32, warning: Vec<String>) -> Self {
        TransformCommandResponse {
            status: CommandStatus::Success,
            num_rows,
            error: Vec::new(),
            warning,
        }
    }

    // Constructor for failure
    pub fn failure(error: Vec<String>) -> Self {
        TransformCommandResponse {
            status: CommandStatus::Failure,
            num_rows: 0,
            error,
            warning: Vec::new(),
        }
    }
}
