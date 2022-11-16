#![allow(dead_code)]
mod sql;
use crate::{database::RB, information::*};
use anyhow::{anyhow, Result};
use function_name::named;
use rbs::to_value;

use sql::SQL;

macro_rules! return_error {
    ($err:expr) => {
        Err(anyhow!(
            "Something error in `{}` with message : {}",
            function_name!(),
            $err
        ))
    };
}

#[named]
/// check a attendance type wether exist by string name
pub async fn query_attendance_type_exist_by_name(attendance_name: String) -> Result<()> {
    match RB
        .fetch(
            &SQL["queryAttendanceTypeExistSql"],
            vec![to_value!(attendance_name)],
        )
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => return_error!(e),
    }
}

#[named]
/// check a attendance type wether exist by usize id
pub async fn query_attendance_type_exist_by_id(attendance_id: usize) -> Result<()> {
    let sql = r#"select attendance_id from attendance_type where attendance_id=? and status=0"#;
    match RB.fetch(sql, vec![to_value!(attendance_id)]).await {
        Ok(_) => Ok(()),
        Err(e) => return_error!(e),
    }
}

#[named]
/// search attendance_type by id
pub async fn search_attendance_type_information_by_id(id: usize) -> Result<Attendance> {
    let sql = r#"select * from attendance_type where attendance_id=?"#;
    match RB.fetch_decode(sql, vec![to_value!(id)]).await {
        Ok(model) => Ok(model),
        Err(e) => return_error!(e),
    }
}

#[named]
/// insert an AttendanceType Information
pub async fn insert_attendance_type_information(
    attendance_name: String,
    base_fine_or_bonus: f32,
    rate_fine_or_bonus: f32,
) -> Result<()> {
    match RB
        .fetch(
            &SQL["insertAttendanceTypeInformationSql"],
            vec![
                to_value!(attendance_name),
                to_value!(base_fine_or_bonus),
                to_value!(rate_fine_or_bonus),
            ],
        )
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => return_error!(err),
    }
}

// #[named]
// async fn delete_attendance_type_information(attendance_id: usize) -> Result<()> {
//     let sql = r#"update attendance_type set status=1 where attendance_id=?"#;
//     match RB.fetch(sql, vec![to_value!(attendance_id)]) {
//         Ok(_) => Ok(()),

//     }
// }
