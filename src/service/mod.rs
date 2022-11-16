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
pub async fn query_attendance_type_exist_by_name(attendance_name: String) -> Result<bool> {
    match RB
        .fetch(
            &SQL["queryAttendanceTypeExistSql"],
            vec![to_value!(attendance_name)],
        )
        .await
    {
        Ok(_) => Ok(true),
        Err(e) => return_error!(e),
    }
}

#[named]
/// check a attendance type wether exist by usize id
pub async fn query_attendance_type_exist_by_id(attendance_id: usize) -> Result<bool> {
    let sql = r#"select attendance_id from attendance_type where attendance_id=? and status=0"#;
    match RB.fetch(sql, vec![to_value!(attendance_id)]).await {
        Ok(_) => Ok(true),
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

#[named]
/// delete attendance type by id
async fn delete_attendance_type_information(attendance_id: usize) -> Result<bool> {
    let sql = r#"update attendance_type set status=1 where attendance_id=?"#;
    match RB.fetch(sql, vec![to_value!(attendance_id)]).await {
        Ok(_) => Ok(true),
        Err(err) => return_error!(err),
    }
}

#[named]
/// query all attendance type
async fn display_attendance_type_information() -> Result<Vec<AttendanceType>> {
    match RB
        .fetch_decode(&SQL["displayAttendanceTypeInformationSql"], vec![])
        .await
    {
        Ok(value) => Ok(value),
        Err(e) => return_error!(e),
    }
}

#[named]
/// insert attendance information
async fn insert_attendance_information(
    employee_id: usize,
    attendance_id: usize,
    time: f32,
) -> Result<()> {
    let sql = r#"insert into attendance(employee_id, attendance_id, attendance_time, record_time)Values(?, ?, ?, now())"#;
    match RB
        .fetch(
            sql,
            vec![
                to_value!(employee_id),
                to_value!(attendance_id),
                to_value!(time),
            ],
        )
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => return_error!(e),
    }
}

#[named]
/// search a staff record after the time
async fn search_attendance_information(id: usize, time: String) -> Result<Vec<Attendance>> {
    match RB
        .fetch_decode(
            &SQL["searchAttendanceInformationSql"],
            vec![to_value!(id), to_value!(time)],
        )
        .await
    {
        Ok(value) => Ok(value),
        Err(e) => return_error!(e),
    }
}

#[named]
/// query a staff the last time he settlement salary
async fn search_max_time_from_salary_record(id: usize) -> Result<String> {
    match RB
        .fetch_decode(
            &SQL["searchMaxTimeFromSalaryRecordSql"],
            vec![to_value!(id)],
        )
        .await
    {
        Ok(value) => Ok(value),
        Err(e) => return_error!(e),
    }
}

#[named]
/// check a political name exits
async fn query_political_exist(political_name: String) -> Result<bool> {
    match RB
        .fetch(
            &SQL["queryPoliticalExistForNameSql"],
            vec![to_value!(political_name)],
        )
        .await
    {
        Ok(_) => Ok(true),
        Err(e) => return_error!(e),
    }
}

#[named]
/// get political id by political name
async fn query_political_id(political_id: String) -> Result<usize> {
    match RB
        .fetch_decode(
            &SQL["queryPoliticalExistForNameSql"],
            vec![to_value!(political_id)],
        )
        .await
    {
        Ok(value) => Ok(value),
        Err(e) => return_error!(e),
    }
}

#[named]
/// insert new political with parameter: political_name
async fn insert_political(political_name: String) -> Result<bool> {
    match RB
        .fetch(&SQL["insertPoliticalSql"], vec![to_value!(political_name)])
        .await
    {
        Ok(_) => Ok(true),
        Err(e) => return_error!(e),
    }
}

#[named]
/// delete political by parameter: political_name
async fn delete_political(political_name: String) -> Result<bool> {
    match RB
        .fetch(
            &SQL["deletePoliticalForNameSql"],
            vec![to_value!(political_name)],
        )
        .await
    {
        Ok(_) => Ok(true),
        Err(e) => return_error!(e),
    }
}

#[named]
/// show all political
async fn display_political_information() -> Result<Vec<Political>> {
    match RB
        .fetch_decode(&SQL["displayPoliticalInformationSql"], vec![])
        .await
    {
        Ok(value) => Ok(value),
        Err(e) => return_error!(e),
    }
}

#[named]
/// show all department
async fn display_department_information() -> Result<Vec<Department>> {
    match RB
        .fetch_decode(&SQL["displayDepartmentInformationSql"], vec![])
        .await
    {
        Ok(value) => Ok(value),
        Err(e) => return_error!(e),
    }
}
#[named]
/// query a department by id
async fn show_department_information(id: usize) -> Result<Department> {
    match RB
        .fetch_decode(
            &SQL["showDepartmentInformationForIdSql"],
            vec![to_value!(id)],
        )
        .await
    {
        Ok(value) => Ok(value),
        Err(e) => return_error!(e),
    }
}
#[named]
/// query department by department_name
async fn query_department_id(department: String) -> Result<usize> {
    match RB.fetch_decode(sql, args)
}