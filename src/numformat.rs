use clap::ValueEnum;
use std::str::FromStr;

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum NumFormat {
    RawBinary,
    U8,
    U16,
    U32,
    U64,
}

impl FromStr for NumFormat {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "raw" => Ok(Self::RawBinary),
            "u8" => Ok(Self::U8),
            "u16" => Ok(Self::U16),
            "u32" => Ok(Self::U32),
            "u64" => Ok(Self::U64),
            _ => Err(()),
        }
    }
}

pub fn print_formatted_value(value: u64, mode: NumFormat) {
    match mode {
        NumFormat::RawBinary => {
            for shift in (0..=56).step_by(8) {
                let byte = ((value >> shift) & 0xFF) as u8;
                print!("{}", byte as char);
            }
        }
        NumFormat::U8 => {
            let bytes: [u8; 8] = value.to_be_bytes();
            for byte in &bytes {
                println!("{byte}");
            }
        }
        NumFormat::U16 => {
            let bytes: [u8; 8] = value.to_be_bytes();
            let u16_values: [u16; 4] = [
                u16::from_be_bytes([bytes[0], bytes[1]]),
                u16::from_be_bytes([bytes[2], bytes[3]]),
                u16::from_be_bytes([bytes[4], bytes[5]]),
                u16::from_be_bytes([bytes[6], bytes[7]]),
            ];
            for u16_value in &u16_values {
                println!("{u16_value}");
            }
        }
        NumFormat::U32 => {
            let bytes: [u8; 8] = value.to_be_bytes();
            let u32_values: [u32; 2] = [
                u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
                u32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]),
            ];
            for u32_value in &u32_values {
                println!("{u32_value}");
            }
        }
        NumFormat::U64 => {
            let bytes: [u8; 8] = value.to_be_bytes();
            let u64_value = u64::from_be_bytes(bytes);
            println!("{u64_value}");
        }
    }
}

pub trait PrintFormattedValue {
    fn print_formatted_value(&self, value: u64);
}

impl PrintFormattedValue for NumFormat {
    fn print_formatted_value(&self, value: u64) {
        print_formatted_value(value, *self);
    }
}
