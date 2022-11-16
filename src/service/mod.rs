#![allow(dead_code)]
use crate::{database::RB, information::*};
use anyhow::{anyhow, Result};
use rbs::to_value;

rbatis::crud!(AttendanceType {});

macro_rules! return_error{
    ($name:expr,$err:expr) => {
        Err(anyhow!(
            "Something error in `{}` with message : {}",
            $err
        ))
    }
}

/// check a attendance type wether exist by string name
async fn query_attendance_type_exist_by_name(attendance_name: String) -> Result<bool> {
    let sql = r#"select * from attendance_type where attendance_id=?"#;
    let res = RB.fetch(sql, vec![to_value!(attendance_name)]).await;
    Ok(res.is_ok())
}

/// check a attendance type wether exist by usize id
async fn query_attendance_type_exist_by_id(attendance_id: usize) -> Result<AttendanceType> {
    let sql = r#"select attendance_id from attendance_type where attendance_id=? and status=0"#;
    match RB.fetch_decode(sql, vec![to_value!(attendance_id)]).await {
        Ok(model) => Ok(model),
        Err(err) => Err(anyhow!(
            "Something error in `query_attendance_type_exist_by_id` with message : {}",
            err
        )),
    }
}

/// insert an AttendanceType Information
async fn insert_attendance_type_information(
    attendance_name: String,
    base_fine_or_bonus: f32,
    rate_fine_or_bonus: f32,
) -> Result<()> {
    let sql = r#"insert into attendance_type(attendance_name, base_fine_or_bonus, rate_fine_or_bonus)values(?, ?, ?)"#;
    match RB.fetch(sql, vec![to_value!(attendance_name), to_value!(base_fine_or_bonus), to_value!(rate_fine_or_bonus)]).await {
        Ok(_) => Ok(()),
        Err(err) => Err(anyhow!(
            "Something error in `insert_AttendanceType_Information` with message : {}",
            err
        ))
    }
}

async fn delete_attendance_type_information(attendance_id: usize) -> Result<bool> {
    let sql = r#"update attendance_type set status=1 where attendance_id=?"#;

}