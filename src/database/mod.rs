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

#[cfg(test)]
mod test {
    use fast_log::Config;

    /// test rbatis wether is able to connect the mysql sever
    /// if success , the test will pass and execute the sql query `show database;`
    /// return a not empty `Option` type;
    #[tokio::test]
    async fn test_rbatis() {
        let _ = fast_log::init(Config::new().file("request.log")).unwrap();
        use crate::database::RB;
        let v = RB.fetch("show databases;", vec![]).await;
        // database is ready to use
        assert!(v.is_ok());
    }
}
