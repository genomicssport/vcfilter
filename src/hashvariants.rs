use std::fs;
use std::io::{BufReader, BufRead};
use std::io::Write;
use std::collections::HashSet;
use crate::variantstruct::Genomecapture;

/*

 Author Gaurav Sablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-5-15

*/

pub fn hashsetref(hashvector:Vec<Genomecapture>) -> Result<HashSet<String>, Box<dyn Error>> {
          let hashsetref:HashSet<String> = HashSet::new();
          let hashvector_borrow = hashvector.clone();
          for i in hashvector_borrow.iter(){
                    hashsetref.insert(i.generef);
          }
          Ok(hashsetref)
}

pub fn hashsetalt(hashvector:Vec<Genomecapture>) -> Results<HashSet<String>, Box<dyn Error>>{
          let hashsetalt:HashSet<String> = HashSet::new();
          let hashvector_alt = hashvector.clone();
          for i in hashvector_alt.iter(){
                    hashsetref.insert(i.alt);
          }
          Ok(hashsetalt)

}
