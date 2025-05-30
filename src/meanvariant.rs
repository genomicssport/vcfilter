use crate::variantstruct::Genomecapture;
use std::error::Error;

/*
 Author Gaurav Sablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-5-15
*/

pub fn meanquality(hashvector: Vec<Genomecapture>) -> Result<usize, Box<dyn Error>> {
    let mut meanquality: Vec<usize> = Vec::new();
    let hashvector_borrow = hashvector;
    for i in hashvector_borrow.iter() {
        meanquality.push(i.qual.clone().parse::<usize>().unwrap());
    }
    let mut meanvariant: usize = 0usize;
    for i in meanquality.iter() {
        meanvariant += i;
    }
    let finallen = meanquality.len();
    Ok(meanvariant / finallen as usize)
}

/*
 for later additionsbut already coded ahead.
pub fn medianquality(hashvector: Vec<Genomecapture>) -> Result<usize, Box<dyn Error>> {
    let mut medianquality: Vec<usize> = Vec::new();
    let hashvector_borrow = hashvector;
    for i in hashvector_borrow.iter() {
        medianquality.push(i.co.clone().parse::<usize>().unwrap());
    }
    let mut medianreturn: usize = 0usize;
    if medianquality.len() / 2 != 0usize {
        let length: usize = medianquality.len() / 2;
        let arrayhalf1: Vec<_> = medianquality[0..length].iter().collect::<Vec<_>>();
        let arrayhalf2: Vec<_> = medianquality[length..medianquality.len()]
            .iter()
            .collect::<Vec<_>>();
        medianreturn += *arrayhalf1[arrayhalf1.len() - 1..arrayhalf1.len()][0]
            + *arrayhalf2[arrayhalf2.len() - 1..arrayhalf2.len()][0] / 2;
    } else if medianquality.len() / 2 == 0usize {
        medianreturn += medianquality[medianquality.len() / 2];
    }
    Ok(medianreturn)
}
*/
