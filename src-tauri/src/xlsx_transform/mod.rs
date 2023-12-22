use log::{error, info};

use self::response::TransformCommandResponse;

mod parser;
pub mod response;
mod writer;

pub async fn transform_xlsx_command(
    src_path: String,
    dest_path: String,
) -> TransformCommandResponse {
    info!("Parse Excel file started for file: {}", src_path);

    let parse_result = parser::parse(&src_path);
    if let Err(error) = &parse_result {
        error!("ERROR: Could not process excel file: {}", error);
        return TransformCommandResponse::failure(vec![error.to_string()]);
    }

    let (raw_data, warnings) = parse_result.unwrap();
    info!("SUCCESS: {} number of rows processed", raw_data.len());

    info!("Creating result file: {}", dest_path);
    if let Err(error) = writer::write(&dest_path, &raw_data) {
        error!("ERROR: Could not create result excel file: {}", error);
        return TransformCommandResponse::failure(vec![error.to_string()]);
    };
    info!("SUCCESS: File created!");

    return TransformCommandResponse::success(raw_data.len() as i32, warnings);
}
