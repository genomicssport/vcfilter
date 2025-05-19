mod args;
mod coveragefilter;
mod defaultfilter;
mod hashvariants;
mod meanvariant;
mod qualityfilter;
mod variantstruct;
mod zygosityfilter;
mod coveragefilter_with_variant;
mod qualityfilter_with_variant;
mod zygosityfilter_with_variant;
mod defaultfilter_with_variant;
mod variantstats;
use crate::args::CommandParse;
use crate::args::Commands;
use crate::coveragefilter::coveragefilteranalysis;
use crate::defaultfilter::defaultfilteranalysis;
use crate::qualityfilter::qualityfilteranalysis;
use crate::zygosityfilter::zygosityfilteranalysis;
use crate::defaultfilter_with_variant::variantdefaultfilteranalysis;
use crate::qualityfilter_with_variant::variantqualityfilteranalysis;
use crate::zygosityfilter_with_variant::variantzygosityfilteranalysis;
use crate::coveragefilter_with_variant::variantcoveragefilteranalysis;
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
        Commands::VariantCoverageVCFFilter { vcffile, coverage, variant } => {
                  let command = variantcoveragefilteranalysis(vcffile, coverage, variant).unwrap();
                  println!("The command has been finished and the file has been written:{}", command);
        }
        Commands::VariantDefaultVCFFilter { vcffile, variant } => {
                  let command = variantdefaultfilteranalysis(vcffile, variant).unwrap();
                  println!("The command has been finished:{}", command);

        }
        Commands::VariantZygosityVCFFilter { vcffile, zygosity, variant } => {
        let command = variantzygosityfilteranalysis(vcffile, zygosity, variant).unwrap();
        println!("The command has finished:{}", command);
    }
    Commands::VariantQualityVCFFilter { vcffile, quality, variant } => {
              let command = variantqualityfilteranalysis(vcffile, quality, variant).unwrap();
              println!("The command has been finished:{}", command);
    }
}
}
