use noodles_bgzf as bgzf;

#[derive(Debug)]
pub struct Chunk {
    start: bgzf::VirtualPosition,
    end: bgzf::VirtualPosition,
}

impl Chunk {
    pub fn new(start: bgzf::VirtualPosition, end: bgzf::VirtualPosition) -> Self {
        Self { start, end }
    }
}
