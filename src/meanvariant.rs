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

pub fn meanquality(hashvector:Vec<Genomcapture>) -> Result<usize, Box<dyn Error>>{
          let hashvector:Vec<usize> = Vec::new();
          let hashvector_borrow:Vec<Genomecapture> = hashvector.clone();
          for i in hashvector_borrow.iter(){
                    hashvector.push(i.qual);
          }
          let meanvariant:usize = hashvector.iter().sum()/hashvector.len();
          Ok(meanvariant)
}
