use ethnum::{U256, u256};
use minustwo::machine::memory::Memory;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn show_memory() {
        let mut memory: Memory = Memory::init(32);
        memory
            .store(0, &[0xAA, 0xBB, 0xCC, 0xDD, 0xF1, 0x66, 172, 254, 209])
            .unwrap();

        println!("{memory}");

        assert_eq!(
            memory.access(0, 6).unwrap(),
            vec![0xAA, 0xBB, 0xCC, 0xDD, 0xF1, 0x66]
        );
    }
}
