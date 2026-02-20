use rsa::BigUint;
use anyhow::{Result, Context};

/// Custom RSA encryption matching JavaScript implementation exactly
pub fn rsa_encrypt_raw(data: &str, pub_exp_hex: &str, modulus_hex: &str) -> Result<String> {
    let e = BigUint::parse_bytes(pub_exp_hex.as_bytes(), 16)
        .context("Failed to parse public exponent (e) from hex")?;
    let n = BigUint::parse_bytes(modulus_hex.as_bytes(), 16)
        .context("Failed to parse modulus (n) from hex")?;

    let modulus_bits = n.bits();
    let chunk_size = ((modulus_bits + 15) / 16) * 2;
    let chunk_size = chunk_size;

    let mut char_codes = Vec::new();
    for ch in data.chars() {
        char_codes.push(ch as u32);
    }

    while char_codes.len() % chunk_size != 0 {
        char_codes.push(0);
    }

    let mut result_parts = Vec::new();

    for chunk in char_codes.chunks(chunk_size) {
        let mut big_int_bytes = Vec::new();

        for i in (0..chunk.len()).step_by(2) {
            let low = chunk.get(i).unwrap_or(&0);
            let high = chunk.get(i + 1).unwrap_or(&0);
            let combined = low + (high << 8);

            big_int_bytes.push((combined & 0xFF_u32) as u8);
            big_int_bytes.push(((combined >> 8) & 0xFF_u32) as u8);
        }

        let m = BigUint::from_bytes_le(&big_int_bytes);

        let encrypted = m.modpow(&e, &n);

        let hex_str = format!("{:x}", encrypted);
        result_parts.push(hex_str);
    }

    let result = result_parts.join(" ");
    Ok(result)
}