use crate::variantstruct::Genomecapture;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::io::Write;
use std::fs::File;

/*

 Author Gaurav Sablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-5-21

*/

pub fn statstable(inputvariant: Vec<Genomecapture>) -> Result<String, Box<dyn Error>> {
    let inputvector: Vec<Genomecapture> = inputvariant.clone();
    let mut veccapture_ref: Vec<String> = Vec::new();
    let mut veccapture_alt: Vec<String> = Vec::new();
    for i in inputvector.iter() {
        veccapture_ref.push(i.generef.clone());
    }
    for i in inputvector.iter() {
        veccapture_alt.push(i.alt.clone());
    }
    let hashcapture_ref: HashSet<String> = veccapture_ref
        .iter()
        .map(|x| x.to_string())
        .collect::<HashSet<_>>();
    let hashcapture_alt: HashSet<String> = veccapture_alt
        .iter()
        .map(|x| x.to_string())
        .collect::<HashSet<_>>();
    let mut outputhashmap_ref: HashMap<(String,String), usize> = HashMap::new();
    let mut outputhashmap_alt: HashMap<(String,String), usize> = HashMap::new();
    for i in hashcapture_ref.iter() {
        for val in inputvector.iter() {
            let inputstring: String = val.generef.to_string();
            let filenameinsert: String = val.filename.to_string();
            let mut count: usize = 0usize;
            if val.generef == inputstring {
                count += 1usize;
            }
            outputhashmap_ref.insert((inputstring,filenameinsert), count);
        }
    }
    for itercapture in hashcapture_alt.iter() {
        for inputval in inputvector.iter() {
            let isolatedstring: String = inputval.alt.to_string();
            let filenameinsert: String = inputval.filename.to_string();
            let mut count: usize = 0usize;
            if inputval.alt == isolatedstring {
                count += 1usize;
            }
            outputhashmap_alt.insert((isolatedstring,filenameinsert), count);
        }
    }

    let mut altstats = File::create("alt-stats").expect("file not present");
    for (i,val) in outputhashmap_alt.iter() {
        writeln!(altstats, "{}\t{}\t{}", i.0, i.1, val);
    }
    let mut refstats = File::create("ref-stats").expect("file not present");
    for (i,val) in outputhashmap_ref.iter() {
        writeln!(refstats, "{}\t{}\t{}", i.0, i.1, val);
    }
    Ok("The stats for the file has been tabulate".to_string())
}
