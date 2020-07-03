extern crate csv;

use std::error::Error;
use std::fs::File;
use std::process;


static CIDR2MASK: &'static [u32] = &[ 0x00000000, 0x80000000,
    0xC0000000, 0xE0000000, 0xF0000000, 0xF8000000, 0xFC000000,
    0xFE000000, 0xFF000000, 0xFF800000, 0xFFC00000, 0xFFE00000,
    0xFFF00000, 0xFFF80000, 0xFFFC0000, 0xFFFE0000, 0xFFFF0000,
    0xFFFF8000, 0xFFFFC000, 0xFFFFE000, 0xFFFFF000, 0xFFFFF800,
    0xFFFFFC00, 0xFFFFFE00, 0xFFFFFF00, 0xFFFFFF80, 0xFFFFFFC0,
    0xFFFFFFE0, 0xFFFFFFF0, 0xFFFFFFF8, 0xFFFFFFFC, 0xFFFFFFFE,
    0xFFFFFFFF];

fn int_to_ipv4(ip:u32) -> String{
    let mut ipv4 :String = String::new();
    let mask_0_8 :u32 = 0x000000FF;
    let mask_9_16 :u32 = 0x0000FF00;
    let mask_17_24 :u32 = 0x00FF0000;
    let mask_25_32 :u32 = 0xFF000000;
    let first_segment :String = ((ip & mask_25_32) >> 24).to_string();
    let second_segment :String = ((ip & mask_17_24) >> 16).to_string();
    let third_segment :String = ((ip & mask_9_16) >> 8).to_string();
    let fourth_segment :String = (ip & mask_0_8).to_string();

    ipv4 = format!("{}{}", ipv4, first_segment);
    ipv4 = format!("{}{}", ipv4, ".");
    ipv4 = format!("{}{}", ipv4, second_segment);
    ipv4 = format!("{}{}", ipv4, ".");
    ipv4 = format!("{}{}", ipv4, third_segment);
    ipv4 = format!("{}{}", ipv4, ".");
    ipv4 = format!("{}{}", ipv4, fourth_segment);

    return ipv4
}

fn ip_range_to_cidr(start:u32, end:u32) -> Vec<String> {
    let mut ip_blocks = Vec::new();
    let mut mutable_start = start;
    while end >= mutable_start {
        let mut max_size :u32 = 32;
        while max_size > 0 {
            let mask :u32 = CIDR2MASK[(max_size - 1) as usize];
            let masked_base :u32 = mutable_start & mask;
            if masked_base != mutable_start {
                break;
            }
            max_size -= 1;
        }
        let max_diff = 32 - integer_log_2(end - mutable_start + 1);
        if max_size < max_diff {
            max_size = max_diff;
        }
        let ip = int_to_ipv4(mutable_start);
        let ip_block = format!("{}{}{}", ip, "/", max_size);
        ip_blocks.push(ip_block);
        if mutable_start as u64 + integer_pow_2(32 - max_size) as u64 >= std::u32::MAX as u64{
            break;
        } else {
            mutable_start += integer_pow_2(32 - max_size);
        }
    }
    return ip_blocks;
}

fn integer_pow_2(x:u32) -> u32 {
    let mut mut_x = x;
    let mut product = 1;
    while mut_x > 0 {
        product *= 2;
        mut_x -= 1;
    }
    return product;
}

fn integer_log_2(x:u32) -> u32 {
    let mut exponent :u32 = 0;
    let mut product :u32 = 1;
    while product < x {
        product *= 2;
        exponent = exponent + 1;
    }
    if product != x {
        return exponent - 1;
    } else {
        return exponent;
    }
}

fn run() -> Result<(), Box<Error>> {
    let file_path = "src/DB.CSV";
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record = result?;
        let start_ip: u32 = record.get(0).unwrap().parse().unwrap();
        let end_ip: u32 = record.get(1).unwrap().parse().unwrap();
        let country_code = record.get(2).unwrap();
        println!("{} {} {}", country_code, start_ip, end_ip);
        let cidr = ip_range_to_cidr(start_ip, end_ip);
        for item in cidr {
            println!("  {}", item);
        }
    }
    Ok(())
}

/// Returns the first positional argument sent to this process. If there are no
/// positional arguments, then this returns an error.
fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}