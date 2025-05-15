mod args;
mod coveragefilter;
mod defaultfilter;
mod hashvariants;
mod meanvariant;
mod qualityfilter;
mod variantstruct;
mod zygosityfilter;
use crate::args::CommandParse;
use crate::args::Commands;
use crate::coveragefilter::coveragefilteranalysis;
use crate::defaultfilter::defaultfilteranalysis;
use crate::qualityfilter::qualityfilteranalysis;
use crate::zygosityfilter::zygosityfilteranalysis;
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
            let command = coveragefilteranalysis(vcffile, &*coverage).unwrap();
            println!("The folder has been filtered:{}", command);
        }
        Commands::QualityVCFFilter { vcffile, quality } => {
            let command = qualityfilteranalysis(vcffile, &*quality).unwrap();
            println!("The folder has been filtered:{}", command);
        }
        Commands::ZygosityVCFFilter { vcffile, zygosity } => {
            let command = zygosityfilteranalysis(vcffile, zygosity).unwrap();
            println!("The folder has been filtered:{}", command);
        }
    }
}
