use std::fmt;

use indexmap::IndexMap;

use super::{builder, tag, Described, Fields, Indexed, Inner, Map, TryFromFieldsError};

type StandardTag = tag::DescribedIndexed;
type Tag = tag::Tag<StandardTag>;

/// An inner VCF header filter map value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Filter {
    description: String,
    idx: Option<usize>,
}

impl Inner for Filter {
    type Id = String;
    type StandardTag = StandardTag;
    type Builder = builder::DescribedIndexed;
}

impl Described for Filter {
    fn description(&self) -> &str {
        &self.description
    }

    fn description_mut(&mut self) -> &mut String {
        &mut self.description
    }
}

impl Indexed for Filter {
    fn idx(&self) -> Option<usize> {
        self.idx
    }

    fn idx_mut(&mut self) -> &mut Option<usize> {
        &mut self.idx
    }
}

impl Map<Filter> {
    /// Creates a default VCF header filter map value for "PASS".
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_vcf::header::record::value::{map::Filter, Map};
    /// let actual = Map::<Filter>::pass();
    /// let expected = Map::<Filter>::new("PASS", "All filters passed");
    /// assert_eq!(actual, expected);
    /// ```
    pub fn pass() -> Self {
        Self {
            id: String::from("PASS"),
            inner: Filter {
                description: String::from("All filters passed"),
                idx: None,
            },
            other_fields: IndexMap::new(),
        }
    }

    /// Creates a VCF header filter map value.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_vcf::header::record::value::{map::Filter, Map};
    /// let map = Map::<Filter>::new("q10", "Quality below 10");
    /// ```
    pub fn new<S, T>(id: S, description: T) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        Self {
            id: id.into(),
            inner: Filter {
                description: description.into(),
                idx: None,
            },
            other_fields: IndexMap::new(),
        }
    }
}

impl fmt::Display for Map<Filter> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        super::fmt_display_prefix(f, self.id())?;
        super::fmt_display_description_field(f, self.description())?;
        super::fmt_display_other_fields(f, self.other_fields())?;

        if let Some(idx) = self.idx() {
            super::fmt_display_idx_field(f, idx)?;
        }

        super::fmt_display_suffix(f)?;

        Ok(())
    }
}

impl TryFrom<Fields> for Map<Filter> {
    type Error = TryFromFieldsError;

    fn try_from(fields: Fields) -> Result<Self, Self::Error> {
        let mut other_fields = super::init_other_fields(fields.len());

        let mut id = None;
        let mut description = None;
        let mut idx = None;

        for (key, value) in fields {
            match Tag::from(key) {
                Tag::Standard(StandardTag::Id) => super::parse_id(&value, &mut id)?,
                Tag::Standard(StandardTag::Description) => {
                    super::parse_description(value, &mut description)?
                }
                Tag::Standard(StandardTag::Idx) => super::parse_idx(&value, &mut idx)?,
                Tag::Other(t) => super::insert_other_field(&mut other_fields, t, value)?,
            }
        }

        let id = id.ok_or(TryFromFieldsError::MissingField("ID"))?;
        let description = description.ok_or(TryFromFieldsError::MissingField("Description"))?;

        Ok(Self {
            id,
            inner: Filter { description, idx },
            other_fields,
        })
    }
}

impl builder::Inner<Filter> for builder::DescribedIndexed {
    fn build(self) -> Result<Filter, builder::BuildError> {
        let description = self
            .description
            .ok_or(builder::BuildError::MissingField("Description"))?;

        Ok(Filter {
            description,
            idx: self.idx,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fmt() {
        let map = Map::<Filter>::pass();
        let expected = r#"<ID=PASS,Description="All filters passed">"#;
        assert_eq!(map.to_string(), expected);
    }

    #[test]
    fn test_try_from_fields_for_map_filter() -> Result<(), TryFromFieldsError> {
        let actual = Map::<Filter>::try_from(vec![
            (String::from("ID"), String::from("PASS")),
            (
                String::from("Description"),
                String::from("All filters passed"),
            ),
        ])?;

        let expected = Map::<Filter>::pass();

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_try_from_fields_for_map_filter_with_missing_fields() {
        assert_eq!(
            Map::<Filter>::try_from(vec![(
                String::from("Description"),
                String::from("All filters passed")
            )]),
            Err(TryFromFieldsError::MissingField("ID")),
        );

        assert_eq!(
            Map::<Filter>::try_from(vec![(String::from("ID"), String::from("PASS")),]),
            Err(TryFromFieldsError::MissingField("Description")),
        );
    }
}
