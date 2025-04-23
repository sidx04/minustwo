use minustwo::machine::memory::Memory;

#[cfg(test)]
mod tests {
    use primitive_types::U256;

    use super::*;

    #[test]
    fn show_memory() {
        let memory: Memory = Memory::init(32);

        println!("Empty memory: {memory}");

        assert_eq!(
            memory.access(0, 6).unwrap(),
            vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
        );
    }

    #[test]
    fn access_and_store_memory() {
        let mut memory: Memory = Memory::init(32);
        memory.store(0, &[0xFF, 0xAA, 0xBB, 0xCC]).unwrap();

        assert_eq!(memory.access(0, 4).unwrap(), &[0xFF, 0xAA, 0xBB, 0xCC]);

        println!("{memory}");
    }

    #[test]
    fn store_memory_as_bytes() {
        let value = U256::from_str_radix(
            "0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF",
            16,
        )
        .unwrap();
        let be_val: [u8; 32] = value.to_big_endian();

        let start_index = be_val.iter().position(|&val| val != 0).unwrap_or(32);
        let res = &be_val[start_index..];
        assert_eq!(*res.last().unwrap(), 0xFF);

        let mut memory = Memory::init(1024);
        memory.store(1, &res).unwrap();
        println!("{memory}");
    }
}
