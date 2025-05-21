use crate::variantstruct::Genomecapture;
use std::collections::HashMap;

/*

 Author Gaurav Sablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-5-21

*/

pub fn statstable(inputvariant: Vec<Genomecapture>) -> Result<String, Box<dyn Error>>{
    let input_vector: Vec<Genomecapture> = inputvariant.clone();
    let veccapture_ref: Vec<String> = Vec::new();
    let veccapture_alt: Vec<String> = Vec::new();
    let outputhashmap_ref: HashMap<String, usize> = HashMap::new();
    let outputhashmap_alt:HashMap<String, usize> = HashMap::new();
    for i in inputvector.iter(){
        veccapture_ref.push(i.generef);
    }
    for i in inputvector.iter(){
        veccapture_alt.push(i.alt);
    }
    for i in veccapture_ref.iter(){
        for val in input_vector.iter(){
        let inputstring: String = val.to_string();
        let count: usize = 0usize;
        if val.generef == inputstring {
            count += 1usize;
        }
        outputhashmap_ref.insert(inputstring, count);
    }
    }
    for itercapture in veccapture_alt.iter(){
        for inputval in input_vector.iter(){
            let isolatedstring:String = inputval.to_string();
            let count: usize = 0usize;
            if inputval.alt == isolatedstring{
                count += 1usize;
            }
            outputhashmap_alt.insert(isolatedstring, count);
        }
    }

    let mut altstats = File::create("alt-stats").expect("file not present");
    for i in outputhashmap_alt.iter(){
        writeln!(altstats, "{}\t{}", i.0, i.1);
    }
    let mut restats = File::create("ref-stats").expect("file not present");
    for i in outputhashmap_ref.iter(){
        writeln!(refstats, "{}\t{}", i.0, i.1);
    }
 Ok("The stats for the file has been tabulate".to_string())

}
