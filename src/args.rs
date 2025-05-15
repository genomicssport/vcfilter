use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "VcFilter",
    version = "1.0",
    about = "vcf filter for human genomics.
       ************************************************
      Gaurav Sablok, IBCH, PAN, Poznan, Poland,
      https://portal.ichb.pl/laboratory-of-genomics/.
      Email: gsablok@ibch.poznan.pl
      Prof. Luiza Handschuh
      Email: luizahan@ibch.poznan.pl.
      ************************************************"
)]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// vcf filter according to quality, coverage and zygosity
    DefaultVCFFilter {
        /// path to the vcf file
        vcffile: String,
    },
    /// vcf filter according to quality
    QualityVCFFilter {
        /// path to the vcf file
        vcffile: String,
        /// quality of the predicted variant
        quality: String,
    },
    /// vcf filter according to coverage
    CoverageVCFFilter {
        /// path to the vcf file
        vcffile: String,
        /// coverage of the predicted variant
        coverage: String,
    },
    /// vcf filter according to zygosity
    ZygosityVCFFilter {
        /// path to the vcf file
        vcffile: String,
        /// zygosity of the predicted variant
        zygosity: String,
    },
}
