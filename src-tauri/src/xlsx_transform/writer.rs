use std::{collections::HashMap, path::PathBuf};

use rust_xlsxwriter::{Formula, Workbook, Worksheet, XlsxError};

use super::parser::RawExcelRow;

lazy_static! {
    // Define a static variable for the month map
    static ref MONTH_MAP: HashMap<i32, &'static str> = {
        let mut map = HashMap::new();
        map.insert(1, "January");
        map.insert(2, "February");
        map.insert(3, "March");
        map.insert(4, "April");
        map.insert(5, "May");
        map.insert(6, "June");
        map.insert(7, "July");
        map.insert(8, "August");
        map.insert(9, "September");
        map.insert(10, "October");
        map.insert(11, "November");
        map.insert(12, "December");
        map
    };
}

fn ensure_xlsx_extension(output_path: &str) -> PathBuf {
    let mut path_buf = PathBuf::from(output_path);

    // Check if the path has the .xlsx extension
    if let Some(extension) = path_buf.extension() {
        if extension != "xlsx" {
            // If not, append .xlsx to the path
            path_buf.set_extension("xlsx");
        }
    } else {
        // If the path has no extension, directly append .xlsx
        path_buf.set_extension("xlsx");
    }

    path_buf
}

fn index_to_excel_column(index: usize) -> String {
    let mut result = String::new();

    // Convert the index to Excel column name
    let mut idx = index + 1; // Adjust for 1-based indexing in Excel

    while idx > 0 {
        let rem = (idx - 1) % 26;
        result.insert(0, (b'A' + rem as u8) as char);
        idx = (idx - rem) / 26;
    }

    result
}

fn create_sheet<'a>(
    workbook: &'a mut Workbook,
    month: &i32,
) -> Result<&'a mut Worksheet, XlsxError> {
    let sheet = workbook.add_worksheet();
    let month_name = MONTH_MAP.get(month).cloned();
    sheet.set_name(month_name.map(|s| s.to_string()).unwrap_or("".to_string()))?;

    sheet.write(0, 0, "Name")?;
    sheet.write(0, 1, "School")?;
    sheet.write(0, 2, "Payment Framework")?;

    for day in 1..=31 {
        sheet.write(0, day + 2, day)?;
    }

    sheet.write(0, 34, "SUM")?;

    Ok(sheet)
}

pub fn write(
    output_path: &str,
    raw_data: &Vec<RawExcelRow>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize a HashMap to store data organized by instructors and months
    let mut data: HashMap<i32, HashMap<String, HashMap<(String, String), HashMap<i32, f64>>>> =
        HashMap::new();

    // Iterate over the rows and organize the data
    for row in raw_data {
        // Entry for the specified month
        let month_entry = data.entry(row.date.month).or_insert_with(HashMap::new);

        // Entry for the specified instructor
        let instructor_entry = month_entry
            .entry(
                row.instructors_name
                    .clone()
                    .unwrap_or("<missing_name>".to_string())
                    .to_string(),
            )
            .or_insert_with(HashMap::new);

        let school_payment_entry = instructor_entry
            .entry((
                row.instructors_school
                    .clone()
                    .unwrap_or("<missing_school>".to_string())
                    .to_string(),
                row.paying_framework.clone(),
            ))
            .or_insert_with(HashMap::new);

        // Entry for the specified day
        let day_entry = school_payment_entry.entry(row.date.day).or_insert(0.0);

        // Add the new training hours to the existing value
        *day_entry += row.training_hours;
    }

    let mut workbook = rust_xlsxwriter::Workbook::new();

    let mut sorted_keys: Vec<_> = data.keys().cloned().collect();
    sorted_keys.sort();

    // Iterate over the instructors and months to build the Excel sheet
    for month in sorted_keys.iter() {
        let monthly_data = data.get(month).unwrap();
        let sheet = create_sheet(&mut workbook, month)?;
        let mut row = 1;

        for (instructor, school_worked_days) in monthly_data {
            for (school_payment, worked_days) in school_worked_days {
                sheet.write(row, 0, instructor)?;
                sheet.write(row, 1, school_payment.0.clone())?;
                sheet.write(row, 2, school_payment.1.clone())?;

                for (day, hours) in worked_days {
                    sheet.write(row, (*day + 2) as u16, *hours)?;
                }

                sheet.write(
                    row,
                    34,
                    Formula::new(format!(
                        "=SUM({}{}:{}{})",
                        index_to_excel_column(3),
                        row + 1,
                        index_to_excel_column(33),
                        row + 1
                    )),
                )?;
                row += 1;
            }
        }
    }

    let output_with_ext = ensure_xlsx_extension(output_path);
    workbook.save(output_with_ext)?;

    Ok(())
}
