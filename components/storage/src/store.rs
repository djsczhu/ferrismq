use rocksdb::DBWithThreadMode;
use rocksdb::MultiThreaded;
use tracing::info;
use rocksdb::Options;
use strum_macros::Display;

#[derive(Debug, Display, PartialEq)]
pub enum DbError {
    NotFound(String),
    IoError(String),
}

pub trait DbStore: Send + Sync + 'static {
    fn get(&self, key: &[u8]) -> Result<Vec<u8>, DbError>;

    fn set(&self, key: &[u8], value: Vec<u8>) -> Result<(), DbError>;

    fn delete(&self, key: &[u8]) -> Result<(), DbError>;

    fn scan(&self, start: &[u8], limit: u32, items: &mut Vec<(Vec<u8>, Vec<u8>)>,
    ) -> Result<(), DbError>;
}

struct RocketDb {
    db: DBWithThreadMode<MultiThreaded>,
}

impl DbStore for RocketDb {
    fn get(&self, key: &[u8]) -> Result<Vec<u8>, DbError> {
        match self.db.get(key) {
            Ok(value) => match value {
                Some(value) => Ok(value),
                None => Err(DbError::NotFound("Not found key".into())),
            }
            Err(err) => Err(DbError::IoError(err.to_string()))
        }
    }

    fn set(&self, key: &[u8], value: Vec<u8>) -> Result<(), DbError> {
        let result = self.db.put(key, value);
        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(DbError::IoError(err.to_string())),
        }
    }

    fn delete(&self, key: &[u8]) -> Result<(), DbError> {
        let result = self.db.delete(key);
        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(DbError::IoError(err.to_string())),
        }
    }

    fn scan(&self, start: &[u8], limit: u32, items: &mut Vec<(Vec<u8>, Vec<u8>)>) -> Result<(), DbError> {
        let it = self.db.iterator(rocksdb::IteratorMode::From(
            start,
            rocksdb::Direction::Forward,
        ));
        for (k, v) in it {

        }
        Ok(())
    }

}

pub fn open_store(dir: &str) -> Result<Box<dyn DbStore>, DbError> {
    info!("Open store: {:?}", dir);
    let mut opts = Options::default();
    opts.create_if_missing(true);
    let db = DBWithThreadMode::<MultiThreaded>::open(&opts, dir);
    match db {
        Ok(db) => {
            Ok(Box::new(RocketDb {db}))
        }
        Err(err) => Err(DbError::IoError(err.to_string())),
    }
}


#[cfg(test)]
mod tests {
    use crate::store::DbError::NotFound;
    use super::*;
    #[test]
    fn test_mutable_db() {
        let db = open_store("/tmp/aaa").unwrap();
        let key = "key1".as_bytes();
        let value2 = "value2";
        //add
        db.set(key.as_ref(), "value1".as_bytes().to_vec()).unwrap();
        //update
        db.set(key.as_ref(), value2.as_bytes().to_vec()).unwrap();
        //retrieve
        let result = db.get(key.as_ref()).unwrap();
        assert_eq!(value2, String::from_utf8(result).unwrap());
        //delete
        db.delete(&key).unwrap();
        assert_eq!(Err(NotFound("Not found key".to_string())), db.get(key.as_ref()));
    }

    #[test]
    fn test_scan_db() {

    }

}