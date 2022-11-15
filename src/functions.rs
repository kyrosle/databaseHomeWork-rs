use once_cell::sync::{Lazy, OnceCell};
use rbatis::{impl_select, Rbatis};
use rbdc_mysql::driver::MysqlDriver;
use tokio::sync::Mutex;

use crate::information::*;

rbatis::crud!(AttendanceType {});
impl_select!(AttendanceType{select_by_id(id:String) -> Option => "`where attendance_id = #{id} limit 1`"});

static CTX: tokio::sync::OnceCell<Mutex<Rbatis>> = tokio::sync::OnceCell::const_new();

async fn database_init() -> Result<Mutex<Rbatis>, rbatis::error::Error> {
    let rb = rbatis::Rbatis::new();
    rb.link(
        MysqlDriver {},
        "mysql://root:1234@localhost:3306/DataBaseHomeWork",
    )
    .await
    .unwrap();
    Ok(Mutex::new(rb))
}

pub async fn init() {
    match CTX.get_or_try_init(database_init).await {
        Ok(_) => {}
        Err(e) => {
            panic!("DataBase Init Err, message here `{}`", e);
        }
    }
}
