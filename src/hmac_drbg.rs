//! Implementation of HMAC deterministic random bit generator.
//!
//! Attempts to implement HMAC DRBG from NIST SP 800-90A Rev. 1. Chapter 10.1.2.
//! <https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-90Ar1.pdf>
//!
//! No reseeding support, or any guarantees that this is correct.

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Reseed interval reached")]
    ReseedIntervalReached,
}
pub type Result<T, E = Error> = std::result::Result<T, E>;

use ring::hmac;

const MAX_RESEED_INTERVAL: u32 = 1_000_000;

pub struct HmacDrbg {
    v: [u8; 32],
    key: hmac::Key,
    reseed_counter: u32,
}

impl HmacDrbg {
    pub fn new(seed: &[u8], personalization_string: &[u8]) -> Self {
        let key = hmac::Key::new(hmac::HMAC_SHA256, seed);
        let mut h = hmac::Context::with_key(&key);
        let mut v = [0x01u8; 32];

        h.update(&v);
        h.update(personalization_string);
        h.update(seed);
        v.copy_from_slice(h.sign().as_ref());

        Self {
            key,
            v,
            reseed_counter: 1,
        }
    }

    /// Returns a vec of random.
    ///
    /// # Example
    ///
    /// ```rust
    /// # fn test() -> Result<(), Box<dyn std::error::Error>> {
    /// use keygen::hmac_drbg::HmacDrbg;
    /// let seed = [0u8; 32]; // change these to something more random
    /// let personalization_string = [0u8; 32];
    /// let mut drbg = HmacDrbg::new(&seed, &personalization_string);
    /// let random_bytes: Vec<u8> = drbg.generate_bytes(80)?;
    /// assert_eq!(random_bytes.len(), 80);
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if reseed interval is reached.
    #[allow(clippy::cast_possible_truncation)]
    pub fn generate_bytes(&mut self, requested_bytes: usize) -> Result<Vec<u8>> {
        if self.reseed_counter > MAX_RESEED_INTERVAL {
            return Err(Error::ReseedIntervalReached);
        }

        let mut random_bytes = Vec::new();
        let mut output = [0u8; 32];

        while random_bytes.len() < requested_bytes {
            let mut h = hmac::Context::with_key(&self.key);
            h.update(&self.v);
            output.copy_from_slice(h.sign().as_ref());
            self.v.copy_from_slice(&output);

            let bytes_to_take = std::cmp::min(32, requested_bytes - random_bytes.len());
            random_bytes.extend_from_slice(&self.v[..bytes_to_take]);
        }

        let mut h = hmac::Context::with_key(&self.key);
        h.update(&self.v);
        output.copy_from_slice(h.sign().as_ref());
        self.v.copy_from_slice(&output);

        self.reseed_counter += requested_bytes as u32;

        Ok(random_bytes)
    }

    /// Returns a slice of random.
    ///
    /// # Example
    ///
    /// ```rust
    /// # fn test() -> Result<(), Box<dyn std::error::Error>> {
    /// use keygen::hmac_drbg::HmacDrbg;
    /// let seed = [0u8; 32]; // change these to something more random
    /// let personalization_string = [0u8; 32];
    /// let mut drbg = HmacDrbg::new(&seed, &personalization_string);
    /// let random_bytes: [u8; 80] = drbg.generate_slice()?;
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if reseed interval is reached.
    #[allow(clippy::cast_possible_truncation)]
    pub fn generate_slice<const N: usize>(&mut self) -> Result<[u8; N]> {
        if self.reseed_counter > MAX_RESEED_INTERVAL {
            return Err(Error::ReseedIntervalReached);
        }

        let mut random_bytes: [u8; N] = [0u8; N];
        let mut l = 0;

        while l < N {
            let mut h = hmac::Context::with_key(&self.key);
            h.update(&self.v);
            self.v.copy_from_slice(h.sign().as_ref());

            let bytes_to_take = std::cmp::min(32, N - l);
            random_bytes[l..l + bytes_to_take].copy_from_slice(&self.v[..bytes_to_take]);
            l += bytes_to_take;
        }

        let mut h = hmac::Context::with_key(&self.key);
        h.update(&self.v);
        self.v.copy_from_slice(h.sign().as_ref());

        self.reseed_counter += N as u32;

        Ok(random_bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hmac_drbg_slice_bytes() -> Result<(), Box<dyn std::error::Error>> {
        let seed = [0u8; 32];
        let personalization_string = [0u8; 32];
        let mut drbg = HmacDrbg::new(&seed, &personalization_string);
        let random_slice: [u8; 80] = drbg.generate_slice()?;

        let mut drbg = HmacDrbg::new(&seed, &personalization_string);
        let random_bytes = drbg.generate_bytes(80)?;
        assert_eq!(random_bytes.len(), 80);
        assert_eq!(random_slice.to_vec(), random_bytes);
        Ok(())
    }
}
