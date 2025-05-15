mod args;
mod coveragefilter;
mod zygosityfilter;
mod defaultfilter;
mod qualityfilter;
mod variantstruct;
mod hashvariants;
mod meanvariant;
use crate::coveragefilter::coveragefilteranalysis;
use crate::zygosityfilter::zygosityfilteranalysis;
use crate::defaultfilter::defaultfilteranalysis;
use crate::qualityfilter::qualityfilteranalysis;
use crate::args::CommandParse;
use crate::args::Commands;
use clap::Parser;

/*

 Author Gaurav Sablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-5-14

*/

fn main() {
    let argsparse = CommandParse::parse();
    match &argsparse.command {
              Commands::DefaultVCFFilter { vcffile } => {
                        let command = defaultfilteranalysis(vcffile).unwrap();
                        println!("The folder has been filtered:{}", command);
              }
              Commands::CoverageVCFFilter { vcffile, coverage } => {
                        let command = coveragefilteranalysis(vcffile, *coverage).unwrap();
                        println!("The folder has been filtered:{}", command);
              }
              Commands::QualityVCFFilter { vcffile, quality } => {
                        let command = qualityfilteranalysis(vcffile, *quality).unwrap();
                        println!("The folder has been filtered:{}", command);
              }
              Commands::ZygosityVCFFilter { vcffile, zygosity } => {
                        let command = zygosityfilteranalysis(vcffile, zygosity).unwrap();
                        println!("The folder has been filtered:{}", command);
              }
    }
}
