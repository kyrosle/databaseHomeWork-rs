use crate::{database::RB, information::*};
use anyhow::Result;
use rbs::to_value;

rbatis::crud!(AttendanceType {});

/// check a attendance type wether exist
async fn query_attendance_type_exist_by_id(attendance_id: usize) -> Result<bool> {
    let sql = r#"select attendance_id from attendance_type where attendance_id=? and status=0"#;

    let res = RB
        .fetch_decode::<AttendanceType>(sql, vec![to_value!(attendance_id)])
        .await;
    Ok(res.is_ok())
}
