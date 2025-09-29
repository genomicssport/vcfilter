# vcfilter

- asynchronous population scale variant filter. It reports stats on the top of the file and gives you the following for each file in the folder:
- sample1.alt-stats: stats on alt allele
- sample1.alt-unique-variants.txt: unique alt allele in that vcf to see after filtering which are the variant types present.
- sample1.filtereds: filtered file according to the variants and coverage or quality or zygosity
- sample1.ref-stats: stat on ref allele
- sample1.ref-unique-variants.txt: unique ref allele in that vcf to see after filtering which are the variant types present.

- [Scientific Rust](https://www.youtube.com/watch?app=desktop&v=dru-2Cn-RTQ)

```
__   __   ___  |  ___| (_) | | | |_    ___   _ __
\ \ / /  / __| | |_    | | | | | __|  / _ \ | '__|
 \ V /  | (__  |  _|   | | | | | |_  |  __/ | |
  \_/    \___| |_|     |_| |_|  \__|  \___| |_|


asynchronous vcf filter for human genomics.
      ************************************************
      Author Gaurav Sablok,
      Email: codeprog@icloud.com
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

Gaurav Sablok \
Instytut Chemii Bioorganicznej \
Polskiej Akademii Nauk \
ul. Noskowskiego 12/14 | 61-704, Poznań \
Poland
