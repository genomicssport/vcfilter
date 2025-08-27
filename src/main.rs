mod args;
mod coveragefilter;
mod coveragefilter_with_variant;
mod defaultfilter;
mod defaultfilter_with_variant;
mod hashvariants;
mod meanvariant;
mod qualityfilter;
mod qualityfilter_with_variant;
mod variantstats;
mod variantstruct;
mod zygosityfilter;
mod zygosityfilter_with_variant;
use crate::args::CommandParse;
use crate::args::Commands;
use crate::coveragefilter::coveragefilteranalysis;
use crate::coveragefilter_with_variant::variantcoveragefilteranalysis;
use crate::defaultfilter::defaultfilteranalysis;
use crate::defaultfilter_with_variant::variantdefaultfilteranalysis;
use crate::qualityfilter::qualityfilteranalysis;
use crate::qualityfilter_with_variant::variantqualityfilteranalysis;
use crate::zygosityfilter::zygosityfilteranalysis;
use crate::zygosityfilter_with_variant::variantzygosityfilteranalysis;
use async_std::task;
use clap::Parser;
use figlet_rs::FIGfont;

/*

 Author Gaurav Sablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-5-14

*/

fn main() {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("vcFilter");
    assert!(figure.is_some());
    println!("{}", figure.unwrap());
    let argsparse = CommandParse::parse();
    match &argsparse.command {
        Commands::DefaultVCFFilter { vcffile } => {
            let command = task::block_on(defaultfilteranalysis(vcffile));
            println!("The folder has been filtered:{:?}", command);
        }
        Commands::CoverageVCFFilter { vcffile, coverage } => {
            let command = task::block_on(coveragefilteranalysis(vcffile, &*coverage));
            println!("The folder has been filtered:{:?}", command);
        }
        Commands::QualityVCFFilter { vcffile, quality } => {
            let command = task::block_on(qualityfilteranalysis(vcffile, &*quality));
            println!("The folder has been filtered:{:?}", command);
        }
        Commands::ZygosityVCFFilter { vcffile, zygosity } => {
            let command = task::block_on(zygosityfilteranalysis(vcffile, zygosity));
            println!("The folder has been filtered:{:?}", command);
        }
        Commands::VariantCoverageVCFFilter {
            vcffile,
            coverage,
            variant,
        } => {
            let command = task::block_on(variantcoveragefilteranalysis(vcffile, coverage, variant));
            println!(
                "The command has been finished and the file has been written:{:?}",
                command
            );
        }
        Commands::VariantDefaultVCFFilter { vcffile, variant } => {
            let command = task::block_on(variantdefaultfilteranalysis(vcffile, variant));
            println!("The command has been finished:{:?}", command);
        }
        Commands::VariantZygosityVCFFilter {
            vcffile,
            zygosity,
            variant,
        } => {
            let command = task::block_on(variantzygosityfilteranalysis(vcffile, zygosity, variant));
            println!("The command has finished:{:?}", command);
        }
        Commands::VariantQualityVCFFilter {
            vcffile,
            quality,
            variant,
        } => {
            let command = task::block_on(variantqualityfilteranalysis(vcffile, quality, variant));
            println!("The command has been finished:{:?}", command);
        }
    }
}
