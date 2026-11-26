/// Standard float rounding helper for determinism.
/// Rounds `val` to `decimals` decimal places, normalizing negative zero to zero.
pub fn round_f64(val: f64, decimals: u32) -> f64 {
    if val.is_nan() || val.is_infinite() {
        return val;
    }
    let factor = 10f64.powi(decimals as i32);
    let rounded = (val * factor).round() / factor;
    if rounded == 0.0 {
        0.0 // Normalize -0.0 to 0.0
    } else {
        rounded
    }
}

/// Convenience helper for 4 decimal places (used by coordinates, opacity, OKLCH, etc.)
pub fn round_4(val: f64) -> f64 {
    round_f64(val, 4)
}

/// Convenience helper for 2 decimal places (used by dimensions, percentages, px values)
pub fn round_2(val: f64) -> f64 {
    round_f64(val, 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rounding_basic() {
        assert_eq!(round_4(1.234567), 1.2346);
        assert_eq!(round_2(1.234567), 1.23);
        assert_eq!(round_f64(-0.00000001, 4), 0.0);
    }
}
