use std::collections::HashMap;

use crate::utils::read_lines;
use once_cell::sync::Lazy;
pub static SQL: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let mut map = HashMap::new();
    match read_lines("./sql.txt") {
        Ok(lines) => {
            for s in lines.flatten() {
                // have {}:{}
                if !s.is_empty() {
                    let sp = s
                        .splitn(2, ':')
                        .map(|v| v.trim().to_string())
                        .collect::<Vec<String>>();

                    if sp.len() == 2 {
                        map.insert(sp[0].to_owned(), sp[1].to_owned());
                    } else {
                        dbg!("line doesn't being split into two part");
                    }
                }
            }
        }
        Err(e) => {
            dbg!("Open File Error : {}", e.to_string());
        }
    }
    map
});


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_sql() {
        assert_eq!(SQL.len(), 44);

        for (k, v) in SQL.iter() {
            assert!(!k.contains(':'));
            assert!(!k.contains('\"'));
            assert!(!k.contains('\''));
            assert!(!k.contains(' '));
            assert!(!v.contains(':'));
            assert!(!v.contains('\"'));
            assert!(!v.contains('\''));
        }
    }
}