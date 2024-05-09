use libmacchina::MemoryReadout;

fn main() {
    use libmacchina::traits::MemoryReadout as _;
    let memory_readout = MemoryReadout::new();

    let total_mem = memory_readout.total().unwrap();
    let used_mem = memory_readout.used().unwrap();
    println!("{used_mem} kB / {total_mem} kB");
}
