// Implement extract_tx_version function below
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    let version_hex: &str = raw_tx_hex
        .get(0..8)
        .ok_or_else(|| "Transaction data too short".to_string())?;

    let mut reversed_version_hex = String::with_capacity(8);

    for i in (0..8).step_by(2).rev() {
        if let Some(byte_pair) = version_hex.get(i..(i + 2)) {
            reversed_version_hex.push_str(byte_pair);
        } else {
            return Err("Internal error: Failed to get byte pair during reversal.".to_string());
        }
    }

    let version = u32::from_str_radix(&reversed_version_hex, 16);
    return version.map_err(|e| format!("Hex decode error"));
}
