use backend::prelude::*;
#[tokio::main]
async fn main() {
    let res =  display_department_information().await;
    println!("{:#?}", res);
}
