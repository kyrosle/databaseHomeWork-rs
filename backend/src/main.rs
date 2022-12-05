use backend::prelude::*;
#[tokio::main]
async fn main() {
    // let _ = fast_log::init(fast_log::Config::new().console()).unwrap();
    let res = search_attendance_information(22, String::from("2000-01-01 00:00:00")).await;
    println!("{:#?}", res);
}
