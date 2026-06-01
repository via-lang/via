use memsizes::Bytes;

pub trait Stats {
    fn reserved_bytes(&self) -> Bytes;
    fn total_bytes(&self) -> Bytes;
    fn used_bytes(&self) -> Bytes;
}
