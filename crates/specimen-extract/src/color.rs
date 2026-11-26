use specimen_core::round_2;

/// Parses a `#RRGGBB` or `#RGB` hex string into `(r, g, b)`.
pub fn parse_hex(hex: &str) -> Option<(u8, u8, u8)> {
    let clean = hex.trim().trim_start_matches('#');
    match clean.len() {
        3 => {
            let r = u8::from_str_radix(&clean[0..1].repeat(2), 16).ok()?;
            let g = u8::from_str_radix(&clean[1..2].repeat(2), 16).ok()?;
            let b = u8::from_str_radix(&clean[2..3].repeat(2), 16).ok()?;
            Some((r, g, b))
        }
        6 => {
            let r = u8::from_str_radix(&clean[0..2], 16).ok()?;
            let g = u8::from_str_radix(&clean[2..4], 16).ok()?;
            let b = u8::from_str_radix(&clean[4..6], 16).ok()?;
            Some((r, g, b))
        }
        _ => None,
    }
}

/// Formats `(r, g, b)` as lowercase `#rrggbb`.
pub fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
    format!("#{:02x}{:02x}{:02x}", r, g, b)
}

/// Calculates relative luminance for WCAG contrast ratio (WCAG 2.1 §1.4.3).
pub fn relative_luminance(r: u8, g: u8, b: u8) -> f64 {
    fn linear(c: u8) -> f64 {
        let v = (c as f64) / 255.0;
        if v <= 0.04045 {
            v / 12.92
        } else {
            ((v + 0.055) / 1.055).powf(2.4)
        }
    }

    0.2126 * linear(r) + 0.7152 * linear(g) + 0.0722 * linear(b)
}

/// Computes WCAG 2.1 contrast ratio between two colors (range: 1.0 to 21.0).
pub fn wcag_contrast_ratio(c1: (u8, u8, u8), c2: (u8, u8, u8)) -> f64 {
    let l1 = relative_luminance(c1.0, c1.1, c1.2);
    let l2 = relative_luminance(c2.0, c2.1, c2.2);

    let (lighter, darker) = if l1 > l2 { (l1, l2) } else { (l2, l1) };
    let ratio = (lighter + 0.05) / (darker + 0.05);
    round_2(ratio)
}

/// Converts sRGB (0-255) to approximate OKLCH [Lightness (0-1), Chroma (0-0.4), Hue (0-360)].
pub fn srgb_to_oklch(r: u8, g: u8, b: u8) -> [f64; 3] {
    // 1. sRGB to linear sRGB
    let r_lin = srgb_to_linear(r);
    let g_lin = srgb_to_linear(g);
    let b_lin = srgb_to_linear(b);

    // 2. Linear sRGB to LMS
    let l = 0.4122214708 * r_lin + 0.5363325363 * g_lin + 0.0514459929 * b_lin;
    let m = 0.2119034982 * r_lin + 0.6806995451 * g_lin + 0.1073969566 * b_lin;
    let s = 0.0883024619 * r_lin + 0.2817188376 * g_lin + 0.6299787005 * b_lin;

    let l_ = l.cbrt();
    let m_ = m.cbrt();
    let s_ = s.cbrt();

    // 3. LMS to Oklab
    let lab_l = 0.2104542553 * l_ + 0.7936177850 * m_ - 0.0040720468 * s_;
    let lab_a = 1.9779984951 * l_ - 2.4285922050 * m_ + 0.4505937099 * s_;
    let lab_b = 0.0259040371 * l_ + 0.7827717662 * m_ - 0.8086757660 * s_;

    // 4. Oklab to Oklch
    let c = (lab_a * lab_a + lab_b * lab_b).sqrt();
    let mut h = lab_b.atan2(lab_a).to_degrees();
    if h < 0.0 {
        h += 360.0;
    }

    [
        specimen_core::round_4(lab_l),
        specimen_core::round_4(c),
        specimen_core::round_2(h),
    ]
}

fn srgb_to_linear(c: u8) -> f64 {
    let v = (c as f64) / 255.0;
    if v <= 0.04045 {
        v / 12.92
    } else {
        ((v + 0.055) / 1.055).powf(2.4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex() {
        assert_eq!(parse_hex("#ffffff"), Some((255, 255, 255)));
        assert_eq!(parse_hex("#000000"), Some((0, 0, 0)));
        assert_eq!(parse_hex("#fff"), Some((255, 255, 255)));
        assert_eq!(parse_hex("7c5cff"), Some((124, 92, 255)));
        assert_eq!(parse_hex("invalid"), None);
    }

    #[test]
    fn test_wcag_contrast() {
        let white = (255, 255, 255);
        let black = (0, 0, 0);
        let ratio = wcag_contrast_ratio(white, black);
        assert_eq!(ratio, 21.0); // Exact maximum WCAG ratio

        let same = wcag_contrast_ratio(white, white);
        assert_eq!(same, 1.0);
    }

    #[test]
    fn test_oklch_conversion() {
        let black_oklch = srgb_to_oklch(0, 0, 0);
        assert_eq!(black_oklch[0], 0.0); // Lightness 0

        let white_oklch = srgb_to_oklch(255, 255, 255);
        assert_eq!(white_oklch[0], 1.0); // Lightness 1
    }
}
