use calamine::{
    open_workbook, DataType, DeError, Range, RangeDeserializerBuilder, Reader, Xls, XlsError, Xlsx,
    XlsxError,
};
use chrono::NaiveDate;
use log::error;
use serde::Deserialize;

const COLUMNS: [&str; 14] = [
    "timestamp",
    "Instructors_email",
    "date",
    "Instructors_name",
    "instructors_school",
    "training_hours",
    "paying_framework",
    "teaching_content",
    "learning_outcomes",
    "atmosphere",
    "technical_problems",
    "conversation_summary",
    "remarks",
    "general_situation",
];

fn read_excel_xlsx(file_path: &str) -> Result<Range<DataType>, XlsxError> {
    return open_workbook::<Xlsx<_>, _>(file_path)?
        .worksheet_range_at(0)
        .ok_or(XlsxError::WorksheetNotFound(
            "Cannot find data worksheet".into(),
        ))?;
}

fn read_excel_xls(file_path: &str) -> Result<Range<DataType>, XlsError> {
    return open_workbook::<Xls<_>, _>(file_path)?
        .worksheet_range_at(0)
        .ok_or(XlsError::WorksheetNotFound(
            "Cannot find data worksheet".into(),
        ))?;
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Date {
    pub month: i32,
    pub day: i32,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RawExcelRow {
    #[serde(deserialize_with = "de_timestamp")]
    pub timestamp: NaiveDate,
    #[serde(rename = "Instructors_email")]
    pub instructors_email: String,
    #[serde(deserialize_with = "de_date")]
    pub date: Date,
    #[serde(
        default,
        rename = "Instructors_name",
        deserialize_with = "de_opt_string"
    )]
    pub instructors_name: Option<String>,
    #[serde(default, deserialize_with = "de_opt_string")]
    pub instructors_school: Option<String>,
    #[serde(deserialize_with = "de_f64")]
    pub training_hours: f64,
    pub paying_framework: String,
    #[serde(default, deserialize_with = "de_opt_string")]
    pub teaching_content: Option<String>,
    #[serde(default, deserialize_with = "de_opt_string")]
    pub learning_outcomes: Option<String>,
    #[serde(default, deserialize_with = "de_opt_string")]
    pub atmosphere: Option<String>,
    #[serde(default, deserialize_with = "de_opt_string")]
    pub technical_problems: Option<String>,
    #[serde(default, deserialize_with = "de_opt_string")]
    pub conversation_summary: Option<String>,
    #[serde(default, deserialize_with = "de_opt_string")]
    pub remarks: Option<String>,
    #[serde(default, deserialize_with = "de_opt_string")]
    pub general_situation: Option<String>,
}

fn de_opt_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let data_type = DataType::deserialize(deserializer);
    match data_type {
        Ok(DataType::Error(_)) => Ok(None),
        Ok(DataType::Float(f)) => Ok(Some(f.to_string())),
        Ok(DataType::Int(i)) => Ok(Some(i.to_string())),
        Ok(DataType::String(s)) => Ok(Some(s)),
        Ok(DataType::DateTime(d)) => Ok(Some(d.to_string())),
        _ => Ok(None),
    }
}

fn de_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let data_type = DataType::deserialize(deserializer);
    match data_type {
        Ok(DataType::Float(f)) => Ok(f),
        Ok(DataType::Int(i)) => Ok(i as f64),
        _ => Ok(0.0),
    }
}

fn de_timestamp<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let data_type = DataType::deserialize(deserializer);
    match data_type {
        Ok(x) => x.as_date().ok_or(serde::de::Error::custom("Invalid Date")),
        Err(x) => Err(x),
    }
}

fn de_date<'de, D>(deserializer: D) -> Result<Date, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let date_str = String::deserialize(deserializer)?;

    // Parse the date string in the "dd/MM" format
    let parts: Vec<&str> = date_str.split('/').collect();
    if parts.len() != 2 {
        return Err(serde::de::Error::custom("Invalid date format"));
    }

    let day: i32 = parts[0].parse().map_err(serde::de::Error::custom)?;
    let month: i32 = parts[1].parse().map_err(serde::de::Error::custom)?;

    Ok(Date { day, month })
}

fn de_error_to_string(error: &DeError) -> String {
    match error {
        DeError::CellOutOfRange { try_pos, min_pos } => {
            format!(
                "Cell out of range - Try position: {:?}, Minimum position: {:?}",
                try_pos, min_pos
            )
        }
        DeError::CellError { err, pos } => {
            format!("Cell error - Error: {:?}, Position: {:?}", err, pos)
        }
        DeError::UnexpectedEndOfRow { pos } => {
            format!("Unexpected end of row - Position: {:?}", pos)
        }
        DeError::HeaderNotFound(header) => format!("Required header not found: {}", header),
        DeError::Custom(custom_err) => format!("{}", custom_err),
    }
}

pub fn parse(
    file_path: &str,
) -> Result<(Vec<RawExcelRow>, Vec<String>), Box<dyn std::error::Error>> {
    let mut data: Vec<RawExcelRow> = Vec::new();
    let mut warnings: Vec<String> = Vec::new();

    let range = if let Ok(xlsx) = read_excel_xlsx(&file_path) {
        xlsx
    } else if let Ok(xls) = read_excel_xls(&file_path) {
        xls
    } else {
        return Err("Invalid file type".into());
    };

    let iter_result =
        RangeDeserializerBuilder::with_headers(&COLUMNS).from_range::<_, RawExcelRow>(&range)?;

    for (index, row) in iter_result.enumerate() {
        match row {
            Ok(row) => {
                data.push(row);
            }
            Err(err) => {
                let msg = format!("Row #{}: {:?}", index + 2, de_error_to_string(&err));
                error!("{}", msg);
                warnings.push(msg);
            }
        }
    }

    return Ok((data, warnings));
}
