use crate::bitvector::BitVector;
use crate::hmac_drbg::HmacDrbg;
use clap::ValueEnum;
use getrandom::getrandom;
use lazy_static::lazy_static;
use std::{arch::asm, fmt};
use std::{
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};
use tiny_keccak::Hasher;
use tiny_keccak::Sha3;
use zeroize::Zeroize;

#[allow(dead_code)]
#[derive(thiserror::Error, Debug)]
/// Errors for random number generation
pub enum Error {
    #[error("RDRAND failed")]
    RdrandFailed,
    #[error("OS random number generation failed")]
    OSRandFailed,
    #[error("Unable to create cpu jitter entropy. System too busy or idle?")]
    CpuJitterFailed,
    #[error("Unable to determine current time")]
    SystemTime(#[from] std::time::SystemTimeError),
    #[error("Time went backwards")]
    BackwardsTimeTravel,
    #[error("Unable to convert bytes to u64")]
    ByteConversion(#[from] std::array::TryFromSliceError),
    #[error("Entropy pool is low {0}")]
    NotEnoughEntropy(u64),
    #[error("Unable to read entropy pool from /proc/sys/kernel/random/entropy_avail")]
    EntropyPoolUnavailable,
    #[error("HMAC DRBG error: {0}")]
    HmacDrbg(#[from] crate::hmac_drbg::Error),
}
pub type Result<T, E = Error> = std::result::Result<T, E>;

fn vec_u8_to_u64(bytes: &[u8]) -> Result<u64> {
    Ok(u64::from_be_bytes(bytes.try_into()?))
}

#[cfg(target_os = "linux")]
fn check_entropy_pool() -> Result<()> {
    const MIN_ENTROPY_THRESHOLD: u64 = 200; // Adjust this threshold as needed

    if let Ok(entropy_avail) = read_to_string("/proc/sys/kernel/random/entropy_avail") {
        let entropy_avail: u64 = entropy_avail.trim().parse().unwrap_or(0);

        if entropy_avail < MIN_ENTROPY_THRESHOLD {
            return Err(Error::NotEnoughEntropy(entropy_avail));
        }
    } else {
        return Err(Error::EntropyPoolUnavailable);
    }
    Ok(())
}

#[allow(clippy::unnecessary_wraps)]
#[cfg(not(target_os = "linux"))]
const fn check_entropy_pool() -> Result<()> {
    // On non-Linux systems (e.g., Windows), we can not check the amount of entropy available.
    Ok(())
}

pub trait Generator {
    fn generate_u64(&self) -> Result<u64>;
}

/// Return U64 random number from the OS.
///
/// On Linux will use `getrandom()` syscall. Fallback to /dev/urandom and /dev/random
/// On Windows will use `BCryptGenRandom()` API.
/// On macOS will use `getentropy()`. Fallback to /dev/urandom
pub struct OsRand {}

impl Generator for OsRand {
    fn generate_u64(&self) -> Result<u64> {
        let mut random_bytes = [0u8; 8];

        check_entropy_pool()?;

        if getrandom(&mut random_bytes).is_ok() {
            let random_u64 = u64::from_le_bytes(random_bytes);
            Ok(random_u64)
        } else {
            Err(Error::OSRandFailed)
        }
    }
}

/// Return U64 random number from the CPU RDRAND instruction.
///
/// If the CPU does not support RDRAND, the function will return an error.
/// This effectively limits the program to only run on Intel & AMD CPUs.
pub struct RdRand {}

impl Generator for RdRand {
    fn generate_u64(&self) -> Result<u64> {
        let mut result: u64;
        let mut success: i8;

        unsafe {
            asm!(
                "rdrand {0}; setc {1}",
                out(reg) result,
                out(reg_byte) success,
            );
        }

        if success != 0 {
            Ok(result)
        } else {
            Err(Error::RdrandFailed)
        }
    }
}

/// Return U64 random number from CPU jitter.
///
/// SHA3 (Keccack) is used to provide a u64 random number from 512 bits of cpujitter entropy bits.
/// Rationale for this is that the cpujitter is not 100% random, but it is still a good source of entropy.
/// Also, using the HMAC DRBG with the current personalization string (*that contains the timestamp*)
/// would result in difficulties when estimating the randomness of the generated random numbers.
pub struct CpuJitter {}

impl Generator for CpuJitter {
    fn generate_u64(&self) -> Result<u64> {
        // Let's take 512 (8 * 64) bits of cpujitter entropy
        let mut combined_data = Vec::new();
        let cpujitterraw = CpuJitterRaw {};
        for _ in 0..8 {
            match cpujitterraw.generate_u64() {
                Ok(raw_value) => {
                    combined_data.extend_from_slice(raw_value.to_be_bytes().as_ref());
                }
                Err(e) => return Err(e),
            }
        }

        // Hash the combined data with SHA3 (Keccak)
        let mut sha3 = Sha3::v256();
        let mut hash_result = [0u8; 32];
        sha3.update(&combined_data);
        sha3.finalize(&mut hash_result);

        // Return the first 64 bits as u64
        vec_u8_to_u64(&hash_result[..8])
    }
}

/// Returns U64 from collected CPU jitter. The amount of raw entropy is around 6bits / byte.
pub struct CpuJitterRaw {}

impl Generator for CpuJitterRaw {
    fn generate_u64(&self) -> Result<u64> {
        let mut bit_vector = BitVector::new();
        let mut loop_count = 0;

        loop {
            let start = std::time::Instant::now();
            let end = std::time::Instant::now();
            let time_diff1 = end.duration_since(start).as_nanos();

            let start = std::time::Instant::now();
            let end = std::time::Instant::now();
            let time_diff2 = end.duration_since(start).as_nanos();

            if time_diff1 != time_diff2 {
                if time_diff1 > time_diff2 {
                    bit_vector.add_bit(true);
                } else {
                    bit_vector.add_bit(false);
                }
            }

            // Check if the BitVector is full or if the loop count has reached maximum number of tries.
            if bit_vector.is_full() {
                break;
            }

            loop_count += 1;
            if loop_count >= 32768 {
                return Err(Error::CpuJitterFailed);
            }
        }

        let result = bit_vector.to_u64();
        Ok(result)
    }
}

lazy_static! {
    static ref PREVIOUS_TIMESTAMP: std::sync::Mutex<(u64, u32)> = std::sync::Mutex::new((0, 0));
}

/// Personalization string combines a fixed string ("kissa123", Finnish for cat123) and both seconds and nanoseconds
/// of current timestamp. This ensures that the personalization string is unique for each call.
fn generate_personalization_string() -> Result<[u8; 32]> {
    let mut personalization_string: [u8; 32] = [0; 32];

    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?;
    let timestamp_secs = timestamp.as_secs();
    let timestamp_nanos = timestamp.subsec_nanos();

    // Compare with previous timestamp
    {
        #[allow(clippy::unwrap_used)]
        let mut prev_timestamp = PREVIOUS_TIMESTAMP.lock().unwrap();
        if timestamp_secs < prev_timestamp.0
            || (timestamp_secs == prev_timestamp.0 && timestamp_nanos <= prev_timestamp.1)
        {
            return Err(Error::BackwardsTimeTravel);
        }

        *prev_timestamp = (timestamp_secs, timestamp_nanos);
    }

    // Copy the string bytes
    let hardcoded_str = "kissa123";
    personalization_string[..hardcoded_str.len()].copy_from_slice(hardcoded_str.as_bytes());

    // Copy the timestamp seconds bytes
    let secs_range = hardcoded_str.len()..hardcoded_str.len() + 8;
    personalization_string[secs_range].copy_from_slice(&timestamp_secs.to_le_bytes());

    // Copy the timestamp nanoseconds bytes
    let nanos_range = hardcoded_str.len() + 8..hardcoded_str.len() + 12;
    personalization_string[nanos_range].copy_from_slice(&timestamp_nanos.to_le_bytes());

    Ok(personalization_string)
}

/// Generate a random u64 combining three different sources
///
/// Generate a 1536 bit seed from three different random number sources.
/// Thats 8 * 64 = 512 bits from each source.
pub struct Combined {}

impl Generator for Combined {
    fn generate_u64(&self) -> Result<u64> {
        let mut seed: Vec<u8> = Vec::new();

        let osrand = OsRand {};
        let rdrand = RdRand {};
        let cpujitter = CpuJitter {};

        for _ in 0..8 {
            seed.extend_from_slice(&osrand.generate_u64()?.to_be_bytes());
            seed.extend_from_slice(&rdrand.generate_u64()?.to_be_bytes());
            seed.extend_from_slice(&cpujitter.generate_u64()?.to_be_bytes());
        }

        // Generate a deterministic, but each time unique, personalization string
        let mut personalization_string: [u8; 32] = generate_personalization_string()?;

        // Generate the u64 random number using HMAC DRBG
        let mut drbg = HmacDrbg::new(&seed, &personalization_string);
        let random_bytes = drbg.generate_bytes(8)?;
        let random_value = vec_u8_to_u64(&random_bytes);

        personalization_string.zeroize();
        seed.zeroize();

        random_value
    }
}

#[derive(ValueEnum, Clone)]
pub enum Source {
    Combined,
    Rdrand,
    Os,
    Cpujitter,
    CpujitterRaw,
}

impl fmt::Debug for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Combined => write!(f, "combined"),
            Self::Rdrand => write!(f, "rdrand"),
            Self::Os => write!(f, "os"),
            Self::Cpujitter => write!(f, "cpujitter"),
            Self::CpujitterRaw => write!(f, "cpujitter-raw"),
        }
    }
}

impl FromStr for Source {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "combined" => Ok(Self::Combined),
            "rdrand" => Ok(Self::Rdrand),
            "os" => Ok(Self::Os),
            "cpujitter" => Ok(Self::Cpujitter),
            "cpujitter-raw" => Ok(Self::CpujitterRaw),
            _ => Err(format!("Unknown source: {s}")),
        }
    }
}
impl Generator for Source {
    fn generate_u64(&self) -> Result<u64> {
        let source: Box<&dyn Generator> = match self {
            Self::Combined => Box::new(&Combined {}),
            Self::Rdrand => Box::new(&RdRand {}),
            Self::Os => Box::new(&OsRand {}),
            Self::Cpujitter => Box::new(&CpuJitter {}),
            Self::CpujitterRaw => Box::new(&CpuJitterRaw {}),
        };
        source.generate_u64()
    }
}
