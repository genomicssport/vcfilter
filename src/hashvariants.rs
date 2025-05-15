use crate::variantstruct::Genomecapture;
use std::collections::HashSet;
use std::error::Error;

/*

 Author Gaurav Sablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-5-15

*/

pub fn hashsetref(hashvector: Vec<Genomecapture>) -> Result<HashSet<String>, Box<dyn Error>> {
    let mut hashsetref: HashSet<String> = HashSet::new();
    let hashvector_borrow = hashvector;
    for i in hashvector_borrow.iter() {
        hashsetref.insert(i.generef.clone());
    }
    Ok(hashsetref)
}

pub fn hashsetalt(hashvector: Vec<Genomecapture>) -> Result<HashSet<String>, Box<dyn Error>> {
    let mut hashsetalt: HashSet<String> = HashSet::new();
    let hashvector_alt = hashvector;
    for i in hashvector_alt.iter() {
        hashsetalt.insert(i.alt.clone());
    }
    Ok(hashsetalt)
}
