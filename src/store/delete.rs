use crate::Store;
use std::error::Error;
use std::io::Write;

impl<'a,K,V> Store<K,V> {

    pub fn delete(&mut self, key: &'a K) -> Result<(), Box<dyn Error>>
    where
    K: PartialEq + Clone + Into<Vec<u8>> + 'a,
    &'a K: Into<Vec<u8>>
    {

        let key_bytes: Vec<u8> = key.into();

        let key_hash = fides::hash::blake_3(&key_bytes);

        match self.graves.iter().find(|&x| x == &key_hash) {

            Some(_) => Ok(()),

            None => {

                self.logs_file.write_all(&[1u8])?;

                self.logs_file.write_all(&key_hash)?;

                self.graves.push(key_hash);

                Ok(())

            }

        }

    }

}
