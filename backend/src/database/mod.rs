use once_cell::sync::Lazy;

use crate::config::CONFIG;

/// Mysql connection Manager
#[allow(dead_code)]
pub static RB: Lazy<rbatis::Rbatis> = Lazy::new(|| {
    let rb = rbatis::Rbatis::new();
    match rb.init(rbdc_mysql::driver::MysqlDriver {}, &CONFIG.mysql_url) {
        Ok(_) => {}
        Err(e) => panic!("!rbatis connect mysql server error!\n{}", e),
    }
    rb
});

#[allow(dead_code)]
pub async fn test_rbatis() -> anyhow::Result<()> {
    let _ = fast_log::init(fast_log::Config::new().file("request.log")).unwrap();
    let v = RB.fetch("show databases;", vec![]).await;
    // database is ready to use
    assert!(v.is_ok());
    Ok(())
}
#[cfg(test)]
mod test {
    /// test rbatis wether is able to connect the mysql sever
    /// if success , the test will pass and execute the sql query `show database;`
    /// return a not empty `Option` type;
    #[tokio::test]
    async fn test_rbatis_warp() {
        super::test_rbatis().await.unwrap();
    }
}
