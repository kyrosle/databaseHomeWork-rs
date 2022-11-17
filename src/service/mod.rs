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
pub async fn delete_attendance_type_information(attendance_id: usize) -> Result<bool> {
    let sql = r#"update attendance_type set status=1 where attendance_id=?"#;
    match RB.fetch(sql, vec![to_value!(attendance_id)]).await {
        Ok(_) => Ok(true),
        Err(err) => return_error!(err),
    }
}

#[named]
/// query all attendance type
pub async fn display_attendance_type_information() -> Result<Vec<AttendanceType>> {
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
pub async fn insert_attendance_information(
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
pub async fn search_attendance_information(id: usize, time: String) -> Result<Vec<Attendance>> {
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
pub async fn search_max_time_from_salary_record(id: usize) -> Result<String> {
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
pub async fn query_political_exist(political_name: String) -> Result<bool> {
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
pub async fn query_political_id(political_id: String) -> Result<usize> {
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
pub async fn insert_political(political_name: String) -> Result<bool> {
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
pub async fn delete_political(political_name: String) -> Result<bool> {
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
pub async fn display_political_information() -> Result<Vec<Political>> {
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
pub async fn display_department_information() -> Result<Vec<Department>> {
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
pub async fn show_department_information(id: usize) -> Result<Department> {
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
pub async fn query_department_id(department_name: String) -> Result<usize> {
    match RB
        .fetch_decode(
            &SQL["queryDepartmentIdForNameSql"],
            vec![to_value!(department_name)],
        )
        .await
    {
        Ok(value) => Ok(value),
        Err(e) => return_error!(e),
    }
}

#[named]
/// check a employee wether is a manager
pub async fn query_department_manager_check(employee_id: usize) -> Result<usize> {
    match RB
        .fetch_decode(
            &SQL["queryDepartmentManagerForIdSql"],
            vec![to_value!(employee_id)],
        )
        .await
    {
        Ok(value) => Ok(value),
        Err(e) => return_error!(e),
    }
}

#[named]
/// check the employee wether is this department manager
pub async fn query_department_manager(employee_id: usize, department_id: usize) -> Result<usize> {
    match RB
        .fetch_decode(
            &SQL["queryDepartmentManagerForManagerIdAndDepartmentIdSql"],
            vec![to_value!(employee_id), to_value!(department_id)],
        )
        .await
    {
        Ok(value) => Ok(value),
        Err(e) => return_error!(e),
    }
}

#[named]
/// query a employee wether belong to this department
pub async fn query_department_employee(employee_id: usize, department_id: usize) -> Result<usize> {
    match RB
        .fetch_decode(
            &SQL["queryDepartmentEmployeeSql"],
            vec![to_value!(employee_id), to_value!(department_id)],
        )
        .await
    {
        Ok(value) => Ok(value),
        Err(e) => return_error!(e),
    }
}

#[named]
/// update a department manager
pub async fn update_department_manager(employee_id: usize, department_id: usize) -> Result<usize> {
    match RB
        .fetch_decode(
            &SQL["updateDepartmentManagerSql"],
            vec![to_value!(employee_id), to_value!(department_id)],
        )
        .await
    {
        Ok(value) => Ok(value),
        Err(e) => return_error!(e),
    }
}

#[named]
/// check the department exist
pub async fn query_department_exist(id: usize) -> Result<usize> {
    match RB
        .fetch_decode(&SQL["queryDepartmentExistSql"], vec![to_value!(id)])
        .await
    {
        Ok(value) => Ok(value),
        Err(e) => return_error!(e),
    }
}

#[named]
/// check wether the department has employee
pub async fn query_department_empty(id: usize) -> Result<bool> {
    match RB
        .fetch_decode::<usize>(&SQL["queryDepartmentEmptySql"], vec![to_value!(id)])
        .await
    {
        Ok(count) if count > 0 => Ok(true),
        Ok(_) => Ok(false),
        Err(e) => return_error!(e),
    }
}

#[named]
/// insert department with department name
pub async fn insert_department_information(department_name: String) -> Result<()> {
    match RB
        .fetch(
            &SQL["insertDepartmentInformationSql"],
            vec![to_value!(department_name)],
        )
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => return_error!(e),
    }
}

#[named]
/// delete department information by id
pub async fn delete_department_information(id: usize) -> Result<()> {
    match RB
        .fetch(&SQL["deleteDepartmentInformationSql"], vec![to_value!(id)])
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => return_error!(e),
    }
}
#[named]
/// check post exist by name
pub async fn query_post_exist_by_name(post_name: String) -> Result<bool> {
    match RB
        .fetch(
            &SQL["deleteDepartmentInformationSql"],
            vec![to_value!(post_name)],
        )
        .await
    {
        Ok(_) => Ok(true),
        Err(e) => return_error!(e),
    }
}

#[named]
/// check post exist by id
pub async fn query_post_exist_by_id(id: usize) -> Result<bool> {
    match RB
        .fetch(&SQL["queryPostExistForIdSql"], vec![to_value!(id)])
        .await
    {
        Ok(_) => Ok(true),
        Err(e) => return_error!(e),
    }
}

#[named]
/// query post by post_name
pub async fn query_post_by_name(post_name: String) -> Result<usize> {
    match RB
        .fetch_decode(&SQL["queryPostExistSql"], vec![to_value!(post_name)])
        .await
    {
        Ok(value) => Ok(value),
        Err(e) => return_error!(e),
    }
}

#[named]
/// insert post information with post_name, salary_id, salary
pub async fn insert_post(post_name: String, salary_id: usize, salary: f32) -> Result<bool> {
    match RB
        .fetch(
            &SQL["insertPostSql"],
            vec![
                to_value!(post_name),
                to_value!(salary_id),
                to_value!(salary),
            ],
        )
        .await
    {
        Ok(_) => Ok(true),
        Err(e) => return_error!(e),
    }
}

#[named]
/// check the name of the post wether being used
pub async fn query_no_employee_use_post(name: String) -> Result<bool> {
    match RB
        .fetch(&SQL["queryNoEmployeeUsePostSql"], vec![to_value!(name)])
        .await
    {
        Ok(_) => Ok(true),
        Err(e) => return_error!(e),
    }
}

#[named]
/// change employee field value by id
pub async fn update_employee_information(
    condition: String,
    val: String,
    id: String,
) -> Result<bool> {
    let sql = format!(
        "{}{}{}",
        SQL["updateEmployeeInformationSqlPre"], condition, SQL["updateEmployeeInformationSqlPost"]
    );
    match RB.fetch(&sql, vec![to_value!(val), to_value!(id)]).await {
        Ok(_) => Ok(true),
        Err(e) => return_error!(e),
    }
}

#[named]
/// delete post by name
pub async fn delete_post_by_name(name: String) -> Result<bool> {
    match RB.fetch(&SQL["deletePostSql"], vec![to_value!(name)]).await {
        Ok(_) => Ok(true),
        Err(e) => return_error!(e),
    }
}

#[named]
/// delete post by id
pub async fn delete_post_by_id(id: String) -> Result<bool> {
    match RB
        .fetch(&SQL["deletePostForIdSql"], vec![to_value!(id)])
        .await
    {
        Ok(_) => Ok(true),
        Err(e) => return_error!(e),
    }
}

#[named]
/// show all post information
pub async fn display_post_information() -> Result<Vec<Post>> {
    match RB
        .fetch_decode(&SQL["displayPostInformationSql"], vec![])
        .await
    {
        Ok(value) => Ok(value),
        Err(e) => return_error!(e),
    }
}

#[named]
#[allow(clippy::too_many_arguments)]
/// insert employee information
pub async fn insert_employee_information(
    post_id: usize,
    department_id: usize,
    name: String,
    birth: String,
    political_id: usize,
    health_status: String,
    salary: f32,
    postscript: String,
) -> Result<usize> {
    let sql = &format!("{}", SQL["insertEmployeeInformationSql"]);
    match RB
        .fetch(
            sql,
            vec![
                to_value!(post_id),
                to_value!(department_id),
                to_value!(name),
                to_value!(birth),
                to_value!(political_id),
                to_value!(health_status),
                to_value!(salary),
                to_value!(postscript),
            ],
        )
        .await
    {
        Ok(_) => match RB.fetch_decode("select @@IDENTITY", vec![]).await {
            Ok(value) => Ok(value),
            Err(e) => return_error!(e),
        },
        Err(e) => return_error!(e),
    }
}

#[named]
/// delete employee information by id
pub async fn delete_employee_information(id: usize) -> Result<bool> {
    match RB
        .fetch(
            &SQL["deleteEmployeeInformationForIdSql"],
            vec![to_value!(id)],
        )
        .await
    {
        Ok(_) => Ok(true),
        Err(e) => return_error!(e),
    }
}

#[named]
/// insert employee-change event with post_id and department_id
pub async fn insert_employee_change_personal(
    employee_id: usize,
    post_id: usize,
    department_id: usize,
) -> Result<bool> {
    match RB
        .fetch(
            &SQL["insertEmployeeChangePersonalSql"],
            vec![
                to_value!(employee_id),
                to_value!(post_id),
                to_value!(department_id),
            ],
        )
        .await
    {
        Ok(_) => Ok(true),
        Err(e) => return_error!(e),
    }
}

#[named]
/// display salary record information by id
pub async fn display_salary_record_information(id: usize) -> Result<Vec<SalaryRecord>> {
    match RB
        .fetch_decode(
            &SQL["displaySalaryRecordInformationSql"],
            vec![to_value!(id)],
        )
        .await
    {
        Ok(value) => Ok(value),
        Err(e) => return_error!(e),
    }
}

#[named]
/// query employee wether is exist
pub async fn query_employee_exist(id: usize) -> Result<bool> {
    match RB
        .fetch(&SQL["queryEmployeeExistForIdSql"], vec![to_value!(id)])
        .await
    {
        Ok(_) => Ok(true),
        Err(e) => return_error!(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    pub async fn test_insert_employee_information() {
        if std::fs::File::open("test.log").is_ok() {
            std::fs::remove_file("test.log").unwrap();
        }
        fast_log::init(fast_log::config::Config::new().file("test.log")).unwrap();
        let pc = match RB
            .fetch_decode::<Vec<PersonalChange>>("SELECT * FROM employee;", vec![])
            .await
        {
            Ok(value) => value,
            Err(e) => panic!("pc error : {}", e),
        };
        let last_id = pc.last().unwrap().id.unwrap() + 1;
        let res = insert_employee_information(
            1,
            1,
            String::from("testman"),
            String::from("2020-07-05"),
            1,
            String::from("hhhh"),
            3000.0,
            String::from("Nothing"),
        )
        .await
        .unwrap();
        assert_eq!(res, last_id);
        let del = delete_employee_information(last_id).await.unwrap();
        assert!(del);
    }
}
