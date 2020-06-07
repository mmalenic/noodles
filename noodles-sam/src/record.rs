//! SAM record and fields

mod builder;
pub mod cigar;
pub mod data;
mod field;
mod flags;
mod mapping_quality;
mod mate_reference_sequence_name;
mod position;
pub mod quality_scores;
mod read_name;
mod reference_sequence_name;
pub mod sequence;

pub use self::{
    builder::Builder, cigar::Cigar, data::Data, field::Field, flags::Flags,
    mapping_quality::MappingQuality, mate_reference_sequence_name::MateReferenceSequenceName,
    position::Position, quality_scores::QualityScores, read_name::ReadName,
    reference_sequence_name::ReferenceSequenceName, sequence::Sequence,
};

use std::{error, fmt, str::FromStr};

pub(crate) const NULL_FIELD: &str = "*";
const FIELD_DELIMITER: char = '\t';
const MAX_FIELDS: usize = 12;

/// A SAM record.
///
/// A SAM record has 11 required fields:
///
///   1. read name (QNAME),
///   2. flags (FLAG),
///   3. reference sequence name (RNAME),
///   4. position (POS),
///   5. mapping quality (MAPQ),
///   6. CIGAR string (CIGAR),
///   7. mate reference sequence name (RNEXT),
///   8. mate position (PNEXT),
///   9. template length (TLEN),
///   10. sequence (SEQ), and
///   11. quality scores (QUAL).
///
/// Additionally, optional data fields can be included with any record.
#[derive(Debug)]
pub struct Record {
    qname: ReadName,
    flag: Flags,
    rname: ReferenceSequenceName,
    pos: Position,
    mapq: MappingQuality,
    cigar: Cigar,
    rnext: MateReferenceSequenceName,
    pnext: Position,
    tlen: i32,
    seq: Sequence,
    qual: QualityScores,
    data: Data,
}

impl Record {
    /// Returns a builder to create a record from each of its fields.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_sam::{self as sam, record::Flags};
    ///
    /// let record = sam::Record::builder()
    ///     .set_read_name("r0".parse()?)
    ///     .set_flags(Flags::UNMAPPED)
    ///     .build();
    ///
    /// assert_eq!(record.read_name().as_ref(), "r0");
    /// assert_eq!(record.flags(), Flags::UNMAPPED);
    /// assert!(record.reference_sequence_name().is_none());
    /// assert!(record.position().is_none());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn builder() -> Builder {
        Builder::new()
    }

    /// Returns the read name of this record.
    ///
    /// This is also called the query name.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_sam as sam;
    ///
    /// let record = sam::Record::default();
    /// assert!(record.read_name().is_empty());
    /// assert_eq!(record.read_name().as_ref(), "*");
    ///
    /// let record = sam::Record::builder().set_read_name("r0".parse()?).build();
    /// assert_eq!(record.read_name().as_ref(), "r0");
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn read_name(&self) -> &ReadName {
        &self.qname
    }

    /// Returns the SAM flags of this record.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_sam::{self as sam, record::Flags};
    ///
    /// let record = sam::Record::default();
    /// assert!(record.flags().is_empty());
    /// assert_eq!(u16::from(record.flags()), 0);
    ///
    /// let record = sam::Record::builder().set_flags(Flags::PAIRED | Flags::READ_1).build();
    /// assert_eq!(record.flags(), Flags::PAIRED | Flags::READ_1);
    /// ```
    pub fn flags(&self) -> Flags {
        self.flag
    }

    /// Returns the reference sequence name of this record.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_sam as sam;
    ///
    /// let record = sam::Record::default();
    /// assert!(record.reference_sequence_name().is_empty());
    /// assert_eq!(record.reference_sequence_name().as_ref(), "*");
    ///
    /// let record = sam::Record::builder().set_reference_sequence_name("sq0".parse()?).build();
    /// assert_eq!(record.reference_sequence_name().as_ref(), "sq0");
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn reference_sequence_name(&self) -> &ReferenceSequenceName {
        &self.rname
    }

    /// Returns the start position of this record.
    ///
    /// This value is 1-based. A position value of 0 is possibly an unmapped read.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_sam::{self as sam, record::Position};
    ///
    /// let record = sam::Record::default();
    /// assert!(record.position().is_none());
    /// assert_eq!(i32::from(record.position()), 0);
    ///
    /// let record = sam::Record::builder().set_position(Position::from(13)).build();
    /// assert_eq!(*record.position(), Some(13));
    /// ```
    pub fn position(&self) -> Position {
        self.pos
    }

    /// Returns the mapping quality of this record.
    ///
    /// Mapping quality ranges from 0 to 254, inclusive. A value of 255 means no mapping quality is
    /// set.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_sam::{self as sam, record::MappingQuality};
    ///
    /// let record = sam::Record::default();
    /// assert!(record.mapping_quality().is_none());
    /// assert_eq!(u8::from(record.mapping_quality()), 255);
    ///
    /// let record = sam::Record::builder().set_mapping_quality(MappingQuality::from(8)).build();
    /// assert_eq!(*record.mapping_quality(), Some(8));
    /// ```
    pub fn mapping_quality(&self) -> MappingQuality {
        self.mapq
    }

    /// Returns the CIGAR operations that describe how the read as mapped.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_sam::{self as sam, record::cigar::{op, Op}};
    ///
    /// let record = sam::Record::default();
    /// assert!(record.cigar().is_empty());
    /// assert_eq!(record.cigar().to_string(), "*");
    ///
    /// let record = sam::Record::builder().set_cigar("34M2S".parse()?).build();
    /// assert_eq!(record.cigar().to_string(), "34M2S");
    /// let mut ops = record.cigar().ops().iter();
    /// assert_eq!(ops.next(), Some(&Op::new(op::Kind::Match, 34)));
    /// assert_eq!(ops.next(), Some(&Op::new(op::Kind::SoftClip, 2)));
    /// assert_eq!(ops.next(), None);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn cigar(&self) -> &Cigar {
        &self.cigar
    }

    /// Returns the reference sequence name of the mate of this record.
    ///
    /// The mate reference sequence name can be empty ("*"), the same as the reference sequence
    /// name ("="), or some other non-empty name.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_sam as sam;
    ///
    /// let record = sam::Record::default();
    /// assert!(record.mate_reference_sequence_name().is_empty());
    /// assert_eq!(record.mate_reference_sequence_name().as_ref(), "*");
    ///
    /// let record = sam::Record::builder()
    ///     .set_mate_reference_sequence_name("=".parse()?)
    ///     .build();
    /// assert!(record.mate_reference_sequence_name().is_eq());
    /// assert_eq!(record.mate_reference_sequence_name().as_ref(), "=");
    ///
    /// let record = sam::Record::builder()
    ///     .set_mate_reference_sequence_name("sq0".parse()?)
    ///     .build();
    /// assert!(record.mate_reference_sequence_name().is_some());
    /// assert_eq!(record.mate_reference_sequence_name().as_ref(), "sq0");
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn mate_reference_sequence_name(&self) -> &MateReferenceSequenceName {
        &self.rnext
    }

    /// Returns the start position of the mate of this record.
    ///
    /// This value is 1-based. A mate position value of 0 is possibly an unmapped mapped.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_sam::{self as sam, record::Position};
    ///
    /// let record = sam::Record::default();
    /// assert!(record.mate_position().is_none());
    /// assert_eq!(i32::from(record.mate_position()), 0);
    ///
    /// let record = sam::Record::builder().set_mate_position(Position::from(21)).build();
    /// assert_eq!(*record.mate_position(), Some(21));
    /// ```
    pub fn mate_position(&self) -> Position {
        self.pnext
    }

    /// Returns the template length of this record.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_sam as sam;
    ///
    /// let record = sam::Record::default();
    /// assert_eq!(record.template_len(), 0);
    ///
    /// let record = sam::Record::builder().set_template_len(101).build();
    /// assert_eq!(record.template_len(), 101);
    /// ```
    pub fn template_len(&self) -> i32 {
        self.tlen
    }

    /// Returns the bases in the sequence of this record.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_sam::{self as sam, record::sequence::Base};
    ///
    /// let record = sam::Record::default();
    /// assert!(record.sequence().is_empty());
    /// assert_eq!(record.sequence().to_string(), "*");
    ///
    /// let record = sam::Record::builder().set_sequence("AT".parse()?).build();
    /// assert_eq!(record.sequence().to_string(), "AT");
    /// let mut bases = record.sequence().bases().iter();
    /// assert_eq!(bases.next(), Some(&Base::A));
    /// assert_eq!(bases.next(), Some(&Base::T));
    /// assert_eq!(bases.next(), None);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn sequence(&self) -> &Sequence {
        &self.seq
    }

    /// Returns the quality score for each base in the sequence.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_sam::{self as sam, record::quality_scores::Score};
    ///
    /// let record = sam::Record::default();
    /// assert!(record.quality_scores().is_empty());
    /// assert_eq!(record.quality_scores().to_string(), "*");
    ///
    /// let record = sam::Record::builder().set_quality_scores("ND".parse()?).build();
    /// assert_eq!(record.quality_scores().to_string(), "ND");
    /// let mut scores = record.quality_scores().scores().iter().copied().map(u8::from);
    /// assert_eq!(scores.next(), Some(45));
    /// assert_eq!(scores.next(), Some(35));
    /// assert_eq!(scores.next(), None);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn quality_scores(&self) -> &QualityScores {
        &self.qual
    }

    /// Returns the optional data fields for this record.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_sam::{self as sam, record::{data, Data}};
    ///
    /// let record = sam::Record::default();
    /// assert!(record.data().is_empty());
    ///
    /// let data = Data::new(vec![data::Field::new(
    ///     data::field::Tag::AlignmentHitCount,
    ///     data::field::Value::Int32(1),
    /// )]);
    /// let record = sam::Record::builder().set_data(data).build();
    /// assert_eq!(record.data().to_string(), "NH:i:1");
    /// ```
    pub fn data(&self) -> &Data {
        &self.data
    }
}

impl Default for Record {
    fn default() -> Self {
        Builder::new().build()
    }
}

#[derive(Debug)]
pub enum ParseError {
    Missing(Field),
    Invalid(Field, String),
}

impl error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Missing(field) => write!(f, "missing field: {}", field.name()),
            Self::Invalid(field, message) => {
                write!(f, "invalid {} field: {}", field.name(), message)
            }
        }
    }
}

impl FromStr for Record {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fields = s.splitn(MAX_FIELDS, FIELD_DELIMITER);

        let qname = parse_string(&mut fields, Field::Name).and_then(|s| {
            s.parse()
                .map_err(|e| ParseError::Invalid(Field::Name, format!("{}", e)))
        })?;

        let flag = parse_u16(&mut fields, Field::Flags).map(Flags::from)?;

        let rname = parse_string(&mut fields, Field::ReferenceSequenceName).and_then(|s| {
            s.parse()
                .map_err(|e| ParseError::Invalid(Field::ReferenceSequenceName, format!("{}", e)))
        })?;

        let pos = parse_i32(&mut fields, Field::Position).map(Position::from)?;
        let mapq = parse_u8(&mut fields, Field::MappingQuality).map(MappingQuality::from)?;

        let cigar = parse_string(&mut fields, Field::Cigar).and_then(|s| {
            s.parse()
                .map_err(|e| ParseError::Invalid(Field::Cigar, format!("{}", e)))
        })?;

        let rnext = parse_string(&mut fields, Field::MateReferenceSequenceName).and_then(|s| {
            s.parse()
                .map_err(|e| ParseError::Invalid(Field::Cigar, format!("{}", e)))
        })?;

        let pnext = parse_i32(&mut fields, Field::MatePosition).map(Position::from)?;
        let tlen = parse_i32(&mut fields, Field::TemplateLength)?;

        let seq = parse_string(&mut fields, Field::Sequence).and_then(|s| {
            s.parse()
                .map_err(|e| ParseError::Invalid(Field::Sequence, format!("{}", e)))
        })?;

        let qual = parse_string(&mut fields, Field::QualityScores).and_then(|s| {
            s.parse()
                .map_err(|e| ParseError::Invalid(Field::QualityScores, format!("{}", e)))
        })?;

        let data = match fields.next() {
            Some(s) => s
                .parse()
                .map_err(|e| ParseError::Invalid(Field::Data, format!("{}", e)))?,
            None => Data::default(),
        };

        Ok(Record {
            qname,
            flag,
            rname,
            pos,
            mapq,
            cigar,
            rnext,
            pnext,
            tlen,
            seq,
            qual,
            data,
        })
    }
}

fn parse_string<'a, I>(fields: &mut I, field: Field) -> Result<&'a str, ParseError>
where
    I: Iterator<Item = &'a str>,
{
    fields.next().ok_or_else(|| ParseError::Missing(field))
}

fn parse_u8<'a, I>(fields: &mut I, field: Field) -> Result<u8, ParseError>
where
    I: Iterator<Item = &'a str>,
{
    parse_string(fields, field).and_then(|s| {
        s.parse()
            .map_err(|e| ParseError::Invalid(field, format!("{}", e)))
    })
}

fn parse_u16<'a, I>(fields: &mut I, field: Field) -> Result<u16, ParseError>
where
    I: Iterator<Item = &'a str>,
{
    parse_string(fields, field).and_then(|s| {
        s.parse()
            .map_err(|e| ParseError::Invalid(field, format!("{}", e)))
    })
}

fn parse_i32<'a, I>(fields: &mut I, field: Field) -> Result<i32, ParseError>
where
    I: Iterator<Item = &'a str>,
{
    parse_string(fields, field).and_then(|s| {
        s.parse()
            .map_err(|e| ParseError::Invalid(field, format!("{}", e)))
    })
}
