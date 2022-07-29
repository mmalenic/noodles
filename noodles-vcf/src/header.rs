//! VCF header and fields.

mod builder;
pub mod file_format;
mod fmt;
pub mod format;
pub mod info;
mod number;
mod parser;
pub mod record;

pub use self::{
    builder::Builder, file_format::FileFormat, number::Number, parser::ParseError, record::Record,
};

use std::str::FromStr;

use indexmap::{IndexMap, IndexSet};

use self::record::value::{
    map::{contig, AlternativeAllele, Contig, Filter, Format, Info, Meta},
    Map,
};

/// VCF header info records.
pub type Infos = IndexMap<info::Key, Map<Info>>;

/// VCF header filter records.
pub type Filters = IndexMap<String, Map<Filter>>;

/// VCF header format records.
pub type Formats = IndexMap<format::Key, Map<Format>>;

/// VCF header alternative allele records.
pub type AlternativeAlleles =
    IndexMap<crate::record::alternate_bases::allele::Symbol, Map<AlternativeAllele>>;

/// VCF header contig records.
pub type Contigs = IndexMap<contig::Name, Map<Contig>>;

/// VCF header sample names.
pub type SampleNames = IndexSet<String>;

/// VCF header generic records.
pub type OtherRecords = IndexMap<record::Key, Vec<record::value::Other>>;

/// VCF header generic records.
#[deprecated(since = "0.18.0", note = "Use `OtherRecords` instead.")]
pub type Records = OtherRecords;

/// A VCF header.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Header {
    file_format: FileFormat,
    infos: Infos,
    filters: Filters,
    formats: Formats,
    alternative_alleles: AlternativeAlleles,
    assembly: Option<String>,
    contigs: Contigs,
    meta: IndexMap<String, Map<Meta>>,
    pedigree_db: Option<String>,
    sample_names: SampleNames,
    other_records: OtherRecords,
}

impl Header {
    /// Returns a builder to create a record from each of its fields.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_vcf as vcf;
    /// let builder = vcf::Header::builder();
    /// ```
    pub fn builder() -> Builder {
        Builder::default()
    }

    /// Returns the file format (`fileformat`) of the VCF.
    ///
    /// `fileformat` is a required meta record and is guaranteed to be set.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_vcf::{self as vcf, header::FileFormat};
    ///
    /// let header = vcf::Header::builder()
    ///     .set_file_format(FileFormat::default())
    ///     .build();
    ///
    /// assert_eq!(header.file_format(), FileFormat::default());
    /// ```
    pub fn file_format(&self) -> FileFormat {
        self.file_format
    }

    /// Returns a mutable reference to the file format (`fileformat`) of the VCF.
    ///
    /// `fileformat` is a required meta record and is guaranteed to be set.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_vcf::{self as vcf, header::FileFormat};
    ///
    /// let mut header = vcf::Header::default();
    ///
    /// let file_format = FileFormat::new(4, 2);
    /// *header.file_format_mut() = file_format;
    ///
    /// assert_eq!(header.file_format(), file_format);
    /// ```
    pub fn file_format_mut(&mut self) -> &mut FileFormat {
        &mut self.file_format
    }

    /// Returns a map of information records (`INFO`).
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_vcf::{
    ///     self as vcf,
    ///     header::{info::Key, record::value::{map::Info, Map}},
    /// };
    ///
    /// let info = Map::<Info>::from(Key::SamplesWithDataCount);
    ///
    /// let header = vcf::Header::builder()
    ///     .add_info(info.clone())
    ///     .build();
    ///
    /// let infos = header.infos();
    /// assert_eq!(infos.len(), 1);
    /// assert_eq!(&infos[0], &info);
    /// ```
    pub fn infos(&self) -> &Infos {
        &self.infos
    }

    /// Returns a mutable reference to a map of information records (`INFO`).
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_vcf::{
    ///     self as vcf,
    ///     header::{info::Key, record::value::{map::Info, Map}},
    /// };
    ///
    /// let mut header = vcf::Header::default();
    ///
    /// let info = Map::<Info>::from(Key::SamplesWithDataCount);
    /// header.infos_mut().insert(info.id().clone(), info);
    ///
    /// let infos = header.infos();
    /// assert_eq!(infos.len(), 1);
    /// assert_eq!(infos[0].id(), &Key::SamplesWithDataCount);
    /// ```
    pub fn infos_mut(&mut self) -> &mut Infos {
        &mut self.infos
    }

    /// Returns a map of filter records (`FILTER`).
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_vcf::{self as vcf, header::record::value::{map::Filter, Map}};
    ///
    /// let filter = Map::<Filter>::new("q10", "Quality below 10");
    ///
    /// let header = vcf::Header::builder()
    ///     .add_filter(filter.clone())
    ///     .build();
    ///
    /// let filters = header.filters();
    /// assert_eq!(filters.len(), 1);
    /// assert_eq!(&filters[0], &filter);
    /// ```
    pub fn filters(&self) -> &Filters {
        &self.filters
    }

    /// Returns a mutable reference to a map of filter records (`FILTER`).
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_vcf::{self as vcf, header::record::value::{map::Filter, Map}};
    ///
    /// let mut header = vcf::Header::default();
    ///
    /// let filter = Map::<Filter>::new("q10", "Quality below 10");
    /// header.filters_mut().insert(filter.id().into(), filter.clone());
    ///
    /// let filters = header.filters();
    /// assert_eq!(filters.len(), 1);
    /// assert_eq!(&filters[0], &filter);
    /// ```
    pub fn filters_mut(&mut self) -> &mut Filters {
        &mut self.filters
    }

    /// Returns a list of genotype format records (`FORMAT`).
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_vcf::{
    ///     self as vcf,
    ///     header::{format::Key, record::value::{map::Format, Map}},
    /// };
    ///
    /// let format = Map::<Format>::from(Key::Genotype);
    ///
    /// let header = vcf::Header::builder()
    ///     .add_format(format.clone())
    ///     .build();
    ///
    /// let formats = header.formats();
    /// assert_eq!(formats.len(), 1);
    /// assert_eq!(&formats[0], &format);
    /// ```
    pub fn formats(&self) -> &Formats {
        &self.formats
    }

    /// Returns a mutable reference to a list of genotype format records (`FORMAT`).
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_vcf::{
    ///     self as vcf,
    ///     header::{format::Key, record::value::{map::Format, Map}},
    /// };
    ///
    /// let mut header = vcf::Header::default();
    ///
    /// let format = Map::<Format>::from(Key::Genotype);
    /// header.formats_mut().insert(format.id().clone(), format.clone());
    ///
    /// let formats = header.formats();
    /// assert_eq!(formats.len(), 1);
    /// assert_eq!(&formats[0], &format);
    /// ```
    pub fn formats_mut(&mut self) -> &mut Formats {
        &mut self.formats
    }

    /// Returns a map of symbolic alternate alleles (`ALT`).
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_vcf::{
    ///     self as vcf,
    ///     header::record::value::{map::AlternativeAllele, Map},
    ///     record::alternate_bases::allele::{
    ///         symbol::{structural_variant::Type, StructuralVariant},
    ///         Symbol,
    ///     },
    /// };
    ///
    /// let alt = Map::<AlternativeAllele>::new(
    ///     Symbol::StructuralVariant(StructuralVariant::from(Type::Deletion)),
    ///     "Deletion",
    /// );
    ///
    /// let header = vcf::Header::builder()
    ///     .add_alternative_allele(alt.clone())
    ///     .build();
    ///
    /// let alternative_alleles = header.alternative_alleles();
    /// assert_eq!(alternative_alleles.len(), 1);
    /// assert_eq!(&alternative_alleles[0], &alt);
    /// ```
    pub fn alternative_alleles(&self) -> &AlternativeAlleles {
        &self.alternative_alleles
    }

    /// Returns a mutable reference to a map of symbolic alternate alleles (`ALT`).
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_vcf::{
    ///     self as vcf,
    ///     header::record::value::{map::AlternativeAllele, Map},
    ///     record::alternate_bases::allele::{
    ///         symbol::{structural_variant::Type, StructuralVariant},
    ///         Symbol,
    ///     },
    /// };
    ///
    /// let mut header = vcf::Header::default();
    ///
    /// let alt = Map::<AlternativeAllele>::new(
    ///     Symbol::StructuralVariant(StructuralVariant::from(Type::Deletion)),
    ///     "Deletion",
    /// );
    /// header.alternative_alleles_mut().insert(alt.id().clone(), alt.clone());
    ///
    /// let alternative_alleles = header.alternative_alleles();
    /// assert_eq!(alternative_alleles.len(), 1);
    /// assert_eq!(&alternative_alleles[0], &alt);
    /// ```
    pub fn alternative_alleles_mut(&mut self) -> &mut AlternativeAlleles {
        &mut self.alternative_alleles
    }

    /// Returns a URI to the breakpoint assemblies (`assembly`) referenced in records.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_vcf as vcf;
    ///
    /// let header = vcf::Header::builder()
    ///     .set_assembly("file:///assemblies.fasta")
    ///     .build();
    ///
    /// assert_eq!(header.assembly(), Some("file:///assemblies.fasta"));
    /// ```
    pub fn assembly(&self) -> Option<&str> {
        self.assembly.as_deref()
    }

    /// Returns a mutable reference to a URI to the breakpoint assemblies (`assembly`) referenced
    /// in records.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_vcf as vcf;
    /// let mut header = vcf::Header::default();
    /// *header.assembly_mut() = Some(String::from("file:///assemblies.fasta"));
    /// assert_eq!(header.assembly(), Some("file:///assemblies.fasta"));
    /// ```
    pub fn assembly_mut(&mut self) -> &mut Option<String> {
        &mut self.assembly
    }

    /// Returns a map of contig records (`contig`).
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_vcf::{self as vcf, header::record::value::{map::Contig, Map}};
    ///
    /// let contig = Map::<Contig>::new("sq0".parse()?);
    ///
    /// let header = vcf::Header::builder()
    ///     .add_contig(contig.clone())
    ///     .build();
    ///
    /// let contigs = header.contigs();
    /// assert_eq!(contigs.len(), 1);
    /// assert_eq!(&contigs[0], &contig);
    /// # Ok::<_, vcf::header::record::value::map::contig::name::ParseError>(())
    /// ```
    pub fn contigs(&self) -> &Contigs {
        &self.contigs
    }

    /// Returns a mutable reference to a map of contig records (`contig`).
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_vcf::{self as vcf, header::record::value::{map::Contig, Map}};
    ///
    /// let mut header = vcf::Header::default();
    ///
    /// let contig = Map::<Contig>::new("sq0".parse()?);
    /// header.contigs_mut().insert(contig.id().clone(), contig.clone());
    ///
    /// let contigs = header.contigs();
    /// assert_eq!(contigs.len(), 1);
    /// assert_eq!(&contigs[0], &contig);
    /// # Ok::<_, vcf::header::record::value::map::contig::name::ParseError>(())
    /// ```
    pub fn contigs_mut(&mut self) -> &mut Contigs {
        &mut self.contigs
    }

    /// Returns a map of meta records (`META`).
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_vcf::{self as vcf, header::record::value::{map::Meta, Map}};
    ///
    /// let meta = Map::<Meta>::new(
    ///     "Assay",
    ///     vec![String::from("WholeGenome"), String::from("Exome")],
    /// );
    ///
    /// let header = vcf::Header::builder()
    ///     .add_meta(meta.clone())
    ///     .build();
    ///
    /// let records = header.meta();
    /// assert_eq!(records.len(), 1);
    /// assert_eq!(&records[0], &meta);
    /// ```
    pub fn meta(&self) -> &IndexMap<String, Map<Meta>> {
        &self.meta
    }

    /// Returns a mutable reference to a map of meta records (`META`).
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_vcf::{self as vcf, header::record::value::{map::Meta, Map}};
    ///
    /// let mut header = vcf::Header::default();
    ///
    /// let meta = Map::<Meta>::new(
    ///     "Assay",
    ///     vec![String::from("WholeGenome"), String::from("Exome")],
    /// );
    /// header.meta_mut().insert(meta.id().into(), meta.clone());
    ///
    /// let records = header.meta();
    /// assert_eq!(records.len(), 1);
    /// assert_eq!(&records[0], &meta);
    /// ```
    pub fn meta_mut(&mut self) -> &mut IndexMap<String, Map<Meta>> {
        &mut self.meta
    }

    /// Returns a URI to the relationships between genomes (`pedigreeDB`).
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_vcf as vcf;
    ///
    /// let header = vcf::Header::builder()
    ///     .set_pedigree_db("file:///pedigree.db")
    ///     .build();
    ///
    /// assert_eq!(header.pedigree_db(), Some("file:///pedigree.db"));
    /// ```
    pub fn pedigree_db(&self) -> Option<&str> {
        self.pedigree_db.as_deref()
    }

    /// Returns a mutable reference to a URI to the relationships between genomes (`pedigreeDB`).
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_vcf as vcf;
    /// let mut header = vcf::Header::default();
    /// *header.pedigree_db_mut() = Some(String::from("file:///pedigree.db"));
    /// assert_eq!(header.pedigree_db(), Some("file:///pedigree.db"));
    /// ```
    pub fn pedigree_db_mut(&mut self) -> &mut Option<String> {
        &mut self.pedigree_db
    }

    /// Returns a list of sample names that come after the FORMAT column in the header record.
    ///
    /// # Examples
    ///
    /// ```
    /// use indexmap::IndexSet;
    /// use noodles_vcf as vcf;
    ///
    /// let header = vcf::Header::builder()
    ///     .add_sample_name("sample0")
    ///     .add_sample_name("sample1")
    ///     .build();
    ///
    /// let expected: IndexSet<_> = [String::from("sample0"), String::from("sample1")]
    ///     .into_iter()
    ///     .collect();
    ///
    /// assert_eq!(header.sample_names(), &expected);
    /// ```
    pub fn sample_names(&self) -> &SampleNames {
        &self.sample_names
    }

    /// Returns a mutable reference to a list of sample names that come after the FORMAT column in
    /// the header record.
    ///
    /// # Examples
    ///
    /// ```
    /// use indexmap::IndexSet;
    /// use noodles_vcf as vcf;
    ///
    /// let mut header = vcf::Header::builder().add_sample_name("sample0").build();
    /// header.sample_names_mut().insert(String::from("sample1"));
    ///
    /// let expected: IndexSet<_> = [String::from("sample0"), String::from("sample1")]
    ///     .into_iter()
    ///     .collect();
    ///
    /// assert_eq!(header.sample_names(), &expected);
    /// ```
    pub fn sample_names_mut(&mut self) -> &mut SampleNames {
        &mut self.sample_names
    }

    /// Returns a map of records with nonstandard keys.
    ///
    /// This includes all records other than `fileformat`, `INFO`, `FILTER`, `FORMAT`, `ALT`,
    /// `assembly`, `contig`, `META`, `SAMPLE`, and `pedigreeDB`.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_vcf::{self as vcf, header::{record::{self, Key}}};
    ///
    /// let key = Key::from("fileDate");
    /// let value = record::value::Other::from("20200709");
    ///
    /// let header = vcf::Header::builder()
    ///     .insert(key.clone(), value.clone())
    ///     .build();
    ///
    /// assert_eq!(
    ///     header.records().first(),
    ///     Some((&key, &vec![value])),
    /// );
    /// ```
    #[deprecated(since = "0.18.0", note = "Use `Header::other_records` instead.")]
    pub fn records(&self) -> &OtherRecords {
        self.other_records()
    }

    /// Returns a map of records with nonstandard keys.
    ///
    /// This includes all records other than `fileformat`, `INFO`, `FILTER`, `FORMAT`, `ALT`,
    /// `assembly`, `contig`, `META`, `SAMPLE`, and `pedigreeDB`.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_vcf::{self as vcf, header::{record::{self, Key}}};
    ///
    /// let key = Key::from("fileDate");
    /// let value = record::value::Other::from("20200709");
    ///
    /// let header = vcf::Header::builder()
    ///     .insert(key.clone(), value.clone())
    ///     .build();
    ///
    /// assert_eq!(
    ///     header.other_records().first(),
    ///     Some((&key, &vec![value])),
    /// );
    /// ```
    pub fn other_records(&self) -> &OtherRecords {
        &self.other_records
    }

    /// Returns a mutable reference to a map of records with nonstandard keys.
    ///
    /// This includes all records other than `fileformat`, `INFO`, `FILTER`, `FORMAT`, `ALT`,
    /// `assembly`, `contig`, `META`, `SAMPLE`, and `pedigreeDB`.
    ///
    /// To simply add an unstructured record, consider using [`Self::insert`] instead.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_vcf::{self as vcf, header::{record::{self, Key}}};
    ///
    /// let key = Key::from("fileDate");
    /// let value = record::value::Other::from("20200709");
    ///
    /// let mut header = vcf::Header::default();
    /// header.records_mut().insert(key.clone(), vec![value.clone()]);
    ///
    /// assert_eq!(
    ///     header.records().first(),
    ///     Some((&key, &vec![value])),
    /// );
    /// ```
    #[deprecated(since = "0.18.0", note = "Use `Header::other_records_mut` instead.")]
    pub fn records_mut(&mut self) -> &mut OtherRecords {
        self.other_records_mut()
    }

    /// Returns a mutable reference to a map of records with nonstandard keys.
    ///
    /// This includes all records other than `fileformat`, `INFO`, `FILTER`, `FORMAT`, `ALT`,
    /// `assembly`, `contig`, `META`, `SAMPLE`, and `pedigreeDB`.
    ///
    /// To simply add an unstructured record, consider using [`Self::insert`] instead.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_vcf::{self as vcf, header::{record::{self, Key}}};
    ///
    /// let key = Key::from("fileDate");
    /// let value = record::value::Other::from("20200709");
    ///
    /// let mut header = vcf::Header::default();
    /// header.other_records_mut().insert(key.clone(), vec![value.clone()]);
    ///
    /// assert_eq!(
    ///     header.other_records().first(),
    ///     Some((&key, &vec![value])),
    /// );
    /// ```
    pub fn other_records_mut(&mut self) -> &mut OtherRecords {
        &mut self.other_records
    }

    /// Returns a header record with the given key.
    ///
    /// This includes all records other than `fileformat`, `INFO`, `FILTER`, `FORMAT`, `ALT`,
    /// `assembly`, `contig`, `META`, `SAMPLE`, and `pedigreeDB`.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_vcf::{self as vcf, header::{record::{self, Key}}};
    ///
    /// let key = Key::from("fileDate");
    /// let value = record::value::Other::from("20200709");
    ///
    /// let header = vcf::Header::builder()
    ///     .insert(key.clone(), value.clone())
    ///     .build();
    ///
    /// assert_eq!(header.get(&key), Some(&[value][..]));
    /// assert!(header.get(&Key::from("reference")).is_none());
    /// ```
    pub fn get(&self, key: &record::Key) -> Option<&[record::value::Other]> {
        self.other_records.get(key).map(|r| &**r)
    }

    /// Inserts a key-value pair representing an unstructured record into the header.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_vcf::{self as vcf, header::{record::{self, Key}}};
    ///
    /// let key = Key::from("fileDate");
    /// let value = record::value::Other::from("20200709");
    ///
    /// let mut header = vcf::Header::default();
    /// assert!(header.get(&key).is_none());
    ///
    /// header.insert(key.clone(), value.clone());
    /// assert_eq!(header.get(&key), Some(&[value][..]));
    /// ```
    pub fn insert(&mut self, key: record::Key, value: record::value::Other) {
        let records = self.other_records.entry(key).or_default();
        records.push(value);
    }
}

impl Default for Header {
    fn default() -> Self {
        Builder::default().build()
    }
}

impl std::fmt::Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}{}={}",
            record::PREFIX,
            record::key::FILE_FORMAT,
            self.file_format()
        )?;

        for info in self.infos().values() {
            writeln!(f, "{}{}={}", record::PREFIX, record::key::INFO, info)?;
        }

        for filter in self.filters().values() {
            writeln!(f, "{}{}={}", record::PREFIX, record::key::FILTER, filter)?;
        }

        for format in self.formats().values() {
            writeln!(f, "{}{}={}", record::PREFIX, record::key::FORMAT, format)?;
        }

        for alternative_allele in self.alternative_alleles().values() {
            writeln!(
                f,
                "{}{}={}",
                record::PREFIX,
                record::key::ALTERNATIVE_ALLELE,
                alternative_allele
            )?;
        }

        if let Some(assembly) = self.assembly() {
            writeln!(
                f,
                "{}{}={}",
                record::PREFIX,
                record::key::ASSEMBLY,
                assembly
            )?;
        }

        for contig in self.contigs().values() {
            writeln!(f, "{}{}={}", record::PREFIX, record::key::CONTIG, contig)?;
        }

        for meta in self.meta().values() {
            writeln!(f, "{}{}={}", record::PREFIX, record::key::META, meta)?;
        }

        if let Some(pedigree_db) = self.pedigree_db() {
            writeln!(
                f,
                "{}{}={}",
                record::PREFIX,
                record::key::PEDIGREE_DB,
                pedigree_db
            )?;
        }

        for (key, values) in &self.other_records {
            for value in values {
                writeln!(f, "{}{}={}", record::PREFIX, key, value)?;
            }
        }

        f.write_str("#CHROM\tPOS\tID\tREF\tALT\tQUAL\tFILTER\tINFO")?;

        if !self.sample_names().is_empty() {
            f.write_str("\tFORMAT")?;

            for sample_name in self.sample_names() {
                write!(f, "\t{}", sample_name)?;
            }
        }

        f.write_str("\n")?;

        Ok(())
    }
}

impl FromStr for Header {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parser::parse(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let header = Header::default();
        assert_eq!(header.file_format(), FileFormat::default());
    }

    #[test]
    fn test_fmt() {
        let header = Header::builder()
            .set_file_format(FileFormat::new(4, 3))
            .add_filter(Map::<Filter>::pass())
            .set_assembly("file:///assemblies.fasta")
            .add_meta(Map::<Meta>::new(
                "Assay",
                vec![String::from("WholeGenome"), String::from("Exome")],
            ))
            .insert(
                record::Key::from("fileDate"),
                record::value::Other::String(String::from("20200514")),
            )
            .build();

        let expected = r#"##fileformat=VCFv4.3
##FILTER=<ID=PASS,Description="All filters passed">
##assembly=file:///assemblies.fasta
##META=<ID=Assay,Type=String,Number=.,Values=[WholeGenome, Exome]>
##fileDate=20200514
#CHROM	POS	ID	REF	ALT	QUAL	FILTER	INFO
"#;

        assert_eq!(header.to_string(), expected);
    }

    #[test]
    fn test_fmt_with_genotypes() {
        let header = Header::builder().add_sample_name("sample0").build();
        let expected = "\
##fileformat=VCFv4.3
#CHROM\tPOS\tID\tREF\tALT\tQUAL\tFILTER\tINFO\tFORMAT\tsample0
";
        assert_eq!(header.to_string(), expected);

        let header = Header::builder()
            .add_sample_name("sample0")
            .add_sample_name("sample1")
            .build();

        let expected = "\
##fileformat=VCFv4.3
#CHROM\tPOS\tID\tREF\tALT\tQUAL\tFILTER\tINFO\tFORMAT\tsample0\tsample1
";

        assert_eq!(header.to_string(), expected);
    }

    #[test]
    fn test_insert_with_duplicate_keys() {
        let key = record::Key::from("noodles");
        let values = [
            record::value::Other::from("0"),
            record::value::Other::from("1"),
        ];

        let mut header = Header::default();

        for value in &values {
            header.insert(key.clone(), value.clone());
        }

        assert_eq!(header.get(&key), Some(&values[..]));
    }
}
