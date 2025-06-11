# vcfilter

![](https://github.com/IBCHgenomic/eVaiutilities/blob/main/logo.png)

- asynchronous population scale variant filter. It reports stats on the top of the file and gives you the following for each file in the folder:
- sample1.alt-stats: stats on alt allele
- sample1.alt-unique-variants.txt: unique alt allele in that vcf to see after filtering which are the variant types present.
- sample1.filtereds: filtered file according to the variants and coverage or quality or zygosity
- sample1.ref-stats: stat on ref allele
- sample1.ref-unique-variants.txt: unique ref allele in that vcf to see after filtering which are the variant types present.

- [Scientific Rust](https://www.youtube.com/watch?app=desktop&v=dru-2Cn-RTQ)

```
asynchronous vcf filter for human genomics.
       ************************************************
      Gaurav Sablok, IBCH, PAN, Poznan, Poland,
      https://portal.ichb.pl/laboratory-of-genomics/.
      Email: gsablok@ibch.poznan.pl
      Prof. Luiza Handschuh
      Email: luizahan@ibch.poznan.pl.
      ************************************************

Usage: vcfilter <COMMAND>

Commands:
  default-vcf-filter           vcf filter according to quality, coverage and zygosity
  quality-vcf-filter           vcf filter according to quality
  coverage-vcf-filter          vcf filter according to coverage
  zygosity-vcf-filter          vcf filter according to zygosity
  variant-default-vcf-filter   vcf filter default quality40, coverage10 and zygosity multi
  variant-quality-vcf-filter   vcf filter according to quality and variant
  variant-coverage-vcf-filter  vcf filter according to coverage and variant
  variant-zygosity-vcf-filter  vcf filter according to zygosity and variant
  help                         Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

```
./target/debug/vcfilter sample-files 47
./target/debug/vcfilter variant-coverage-vcf-filter sample-files 47 A
```

- To install windows version:
```
rustup component add llvm-tools
rustup target add x86_64-pc-windows-msvc
git clone https://github.com/IBCHgenomic/vcfilter.git
cd ensemblcov
cargo xwin build --target x86_64-pc-windows-msvc
```

- Acknowledgements: MOSAIC platform, developed as part of the ECBiG-MOSAIC project (POIR.04.02.00-00-D017/20), co-financed by the European Regional Development Fund (ERDF) under the Smart Growth Operational Programme 2014-2020, Measure 4.2 for the development of modern research infrastructure in the science sector.
- Project PI and Informal queries: Prof. Luiza Handschuh: luizahan@ibch.poznan.pl.
- Code related queries: Dr. Gaurav Sablok: gsablok@ibch.poznan.pl.

Gaurav Sablok Instytut Chemii Bioorganicznej Polskiej Akademii Nauk ul. Noskowskiego 12/14 | 61-704, Poznań Poland
