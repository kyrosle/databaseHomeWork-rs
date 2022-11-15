use databaseHomeWork_rs::information::AttendanceType;
use fast_log::{print, init};
use rbdc_mysql::driver::MysqlDriver;

#[tokio::main]
async fn main() {
    let data = AttendanceType::select_by_id(&mut rb, "2".to_string()).await;
    println!("{:#?}", data);
}
