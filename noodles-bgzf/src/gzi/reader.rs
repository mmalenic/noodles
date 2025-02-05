use std::io::{self, Read};

use byteorder::{LittleEndian, ReadBytesExt};

use super::Index;

/// A gzip index (GZI) reader.
pub struct Reader<R> {
    inner: R,
}

impl<R> Reader<R>
where
    R: Read,
{
    /// Creates a gzip index (GZI) reader.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_bgzf::gzi;
    /// let data = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    /// let reader = gzi::Reader::new(&data[..]);
    /// ```
    pub fn new(inner: R) -> Self {
        Self { inner }
    }

    /// Reads a gzip index.
    ///
    /// The position of the stream is expected to be at the start.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::{fs::File, io};
    /// use noodles_bgzf::gzi;
    /// let mut reader = File::open("in.gzi").map(gzi::Reader::new)?;
    /// let index = reader.read_index()?;
    /// # Ok::<(), io::Error>(())
    /// ```
    pub fn read_index(&mut self) -> io::Result<Index> {
        let len = self.inner.read_u64::<LittleEndian>().and_then(|n| {
            usize::try_from(n).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        })?;

        let mut offsets = Vec::with_capacity(len);

        for _ in 0..len {
            let compressed = self.inner.read_u64::<LittleEndian>()?;
            let uncompressed = self.inner.read_u64::<LittleEndian>()?;
            offsets.push((compressed, uncompressed));
        }

        match self.inner.read_u8() {
            Ok(_) => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "unexpected trailing data",
            )),
            Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => Ok(offsets),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_index() -> io::Result<()> {
        let data = [
            0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // len = 2
            0x3c, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // compressed_offset = 4668
            0x2e, 0x53, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // uncompressed_offset = 21294
            0x02, 0x5d, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // compressed_offset = 23810
            0x01, 0x52, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, // uncompressed_offset = 86529
        ];

        let mut reader = Reader::new(&data[..]);
        assert_eq!(reader.read_index()?, vec![(4668, 21294), (23810, 86529)]);

        Ok(())
    }

    #[test]
    fn test_read_index_with_no_entries() -> io::Result<()> {
        let data = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]; // len = 0

        let mut reader = Reader::new(&data[..]);
        let index = reader.read_index()?;
        assert!(index.is_empty());

        Ok(())
    }

    #[test]
    fn test_read_index_with_fewer_than_len_entries() -> io::Result<()> {
        let data = [
            0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // len = 3
            0x3c, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // compressed_offset = 4668
            0x2e, 0x53, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // uncompressed_offset = 21294
            0x02, 0x5d, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // compressed_offset = 23810
            0x01, 0x52, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, // uncompressed_offset = 86529
        ];

        let mut reader = Reader::new(&data[..]);

        assert!(matches!(
            reader.read_index(),
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof
        ));

        Ok(())
    }

    #[test]
    fn test_read_index_with_trailing_data() -> io::Result<()> {
        let data = [
            0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // len = 1
            0x3c, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // compressed_offset = 4668
            0x2e, 0x53, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // uncompressed_offset = 21294
            0x00,
        ];

        let mut reader = Reader::new(&data[..]);

        assert!(matches!(
            reader.read_index(),
            Err(e) if e.kind() == io::ErrorKind::InvalidData
        ));

        Ok(())
    }
}
