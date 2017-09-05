use std::io::{Read, Seek, SeekFrom};
use std::fs::File;

const NUM_CORES: u32 = 4; // XXX

fn open_msr_nodes(num_cores: u32) -> Vec<File> {
    let mut nodes = Vec::new();
    for i in 0..num_cores {
        let path = format!("/dev/cpu/{}/msr", i);
        nodes.push(File::open(path).expect("failed to open msr node"));
    }
    nodes
}

fn read_msr(msr_node: &mut File, msr_addr: u64) -> Result<[u8; 8], ()> {
    let mut buf = [0; 8];
    let seek_addr = SeekFrom::Start(msr_addr);
    msr_node.seek(seek_addr).expect("failed to seek");
    let res = msr_node.read_exact(&mut buf);
    if res.is_err() {
        return Err(());
    }
    Ok(buf)
}

fn print_msr(msr_node: &mut File, msr_addr: u64) {
    let buf = read_msr(msr_node, msr_addr).expect("failed to read msr");
    print_msr_val(&buf);
}

fn print_msr_val(buf: &[u8; 8]) {
    for byte in (0..buf.len()).rev() {
        if byte == 3 {
            print!("  ");
        }
        print!("{:08b} ", buf[byte]);
    }
    print!("      ");
    for byte in (0..buf.len()).rev() {
        if byte == 3 {
            print!("  ");
        }
        print!("{:02x} ", buf[byte]);
    }
}

fn flip_bits(msr_nodes: &mut Vec<File>, msr_addr: u64, bits: Vec<u8>) {
    for bit in bits {
        // XXX
    }
}

fn parse_msr_addr(input: &str) -> Option<u64> {
    let ret;
    if input.starts_with("0x") {
        // hex
        ret = u64::from_str_radix(&input[2..], 16).ok();
    } else {
        // decimal
        ret = input.parse().ok();
    }
    ret
}

fn main() {
    let mut args = std::env::args();
    let _ = args.next();
    let msr_addr_arg = args.next();
    if msr_addr_arg.is_none() {
        println!("usage: XXX");
        return;
    }
    let msr_addr_str = msr_addr_arg.unwrap();

    let mut msr_nodes = open_msr_nodes(NUM_CORES);
    let msr_addr = parse_msr_addr(&msr_addr_str);
    if msr_addr.is_none() {
        println!("failed to parse msr address: {}", msr_addr_str);
        return;
    }
    let msr_addr = msr_addr.unwrap();

    // parse bits
    let bits: Vec<u8> = Vec::new();
    for bit_str in args {
        let bit: Option<u8> = bit_str.parse().ok();
        if bit.is_none() {
            println!("Bad bit: {}", bit_str);
            return;
        }
    }
    flip_bits(&mut msr_nodes, msr_addr, bits);
    for (core, mut msr_node) in msr_nodes.iter_mut().enumerate() {
        print!("{}: ", core);
        print_msr(&mut msr_node, msr_addr);
        println!();
    }
}
