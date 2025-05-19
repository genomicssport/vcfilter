use crate::variantstruct::Genomecapture;
use std::collections::HashMap;

/*

 Author Gaurav Sablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-5-19

*/

pub fn statstable(inputvariant: Vec<Genomecapture>) -> Result<(HashMap<String, usize>,HashMap<String, usize>), Box<dyn Error>>{
    let input_vector: Vec<Genomecapture> = inputvariant.clone();
    let outputhashmap: HashMap<String, usize> = HashMap::new();


}
