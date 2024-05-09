use libmacchina::MemoryReadout;

fn main() {
    use libmacchina::traits::MemoryReadout as _;
    let memory_readout = MemoryReadout::new();

    dbg!(memory_readout.total().unwrap());
    dbg!(memory_readout.free().unwrap());
    dbg!(memory_readout.reclaimable().unwrap());
    dbg!(memory_readout.used().unwrap());
}
