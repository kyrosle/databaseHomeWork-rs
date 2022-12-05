use rbatis::rbdc;
use serde::{Deserialize, Serialize};

// Dto Fields :
pub type FieldString = Option<String>;
pub type FieldFloat = Option<rbdc::decimal::Decimal>;
pub type Key = Option<usize>;
pub type Id = Option<usize>;
pub type CheckStatus = Option<usize>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Attendance {
    pub name: FieldString,
    pub attendance_name: FieldString,
    pub attendance_time: FieldFloat,
    pub base_fine_or_bonus: FieldFloat,
    pub rate_fine_or_bonus: FieldFloat,
    pub record_time: FieldString,
}

impl Attendance {
    pub fn get_salary(&self) -> f32 {
        let base_fine_or_bonus = self
            .base_fine_or_bonus
            .clone()
            .unwrap()
            .0
            .parse::<f32>()
            .unwrap();
        let rate_fine_or_bonus = self
            .rate_fine_or_bonus
            .clone()
            .unwrap()
            .0
            .parse::<f32>()
            .unwrap();
        let attendance_time = self
            .attendance_time
            .clone()
            .unwrap()
            .0
            .parse::<f32>()
            .unwrap();
        base_fine_or_bonus + rate_fine_or_bonus * attendance_time
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AttendanceType {
    pub attendance_id: Key,
    pub attendance_name: FieldString,
    pub base_fine_or_bonus: FieldFloat,
    pub rate_fine_or_bonus: FieldFloat,
    pub status: CheckStatus,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Department {
    pub id: Key,
    pub manager_id: Id,
    pub name: FieldString,
    pub manager_name: FieldString,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Employee {
    pub id: Key,
    pub post_id: Id,
    pub name: FieldString,
    pub birth: FieldString,
    pub postscript: FieldString,
    pub salary: FieldFloat,
    pub department_id: Id,
    pub political_id: Id,
    pub post_name: FieldString,
    pub department_name: FieldString,
    pub political_name: FieldString,
    pub health_status: FieldString,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PersonalChange {
    pub id: Key,
    pub employee_id: Id,
    pub post_id: Id,
    pub department_id: Id,
    pub employee_name: FieldString,
    pub post_name: FieldString,
    pub department_name: FieldString,
    pub change_time: FieldString,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Political {
    pub id: Key,
    pub name: FieldString,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: Key,
    pub salary_id: Id,
    pub name: FieldString,
    pub salary: FieldFloat,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SalaryRecord {
    pub id: Key,
    pub employee_id: Id,
    pub employee_name: FieldString,
    pub salary: FieldFloat,
    pub basic_salary: FieldFloat,
    pub bonus: FieldFloat,
    pub fine: FieldFloat,
    pub starting_time: FieldString,
    pub cut_of_time: FieldString,
}
