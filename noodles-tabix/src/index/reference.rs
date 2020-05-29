pub mod bin;

pub use self::bin::Bin;

use noodles_bgzf as bgzf;

#[derive(Debug)]
pub struct Reference {
    bins: Vec<Bin>,
    intervals: Vec<bgzf::VirtualPosition>,
}

impl Reference {
    pub fn new(bins: Vec<Bin>, intervals: Vec<bgzf::VirtualPosition>) -> Self {
        Self { bins, intervals }
    }

    pub fn bins(&self) -> &[Bin] {
        &self.bins
    }

    pub fn intervals(&self) -> &[bgzf::VirtualPosition] {
        &self.intervals
    }
}
