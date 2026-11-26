use serde::Serialize;
use serde_json::{Number, Value};
use std::collections::BTreeMap;

use crate::error::{CoreError, Result};
use crate::rounding::round_4;

/// Canonicalizes a `serde_json::Value`:
/// - Object keys are sorted lexicographically via `BTreeMap`.
/// - Floats are rounded to 4 decimal places with `-0.0` converted to `0.0`.
/// - Recursively applies to arrays and nested objects.
pub fn canonicalize_value(val: &Value) -> Value {
    match val {
        Value::Object(map) => {
            let mut sorted = BTreeMap::new();
            for (k, v) in map {
                sorted.insert(k.clone(), canonicalize_value(v));
            }
            Value::Object(sorted.into_iter().collect())
        }
        Value::Array(arr) => {
            let normalized: Vec<Value> = arr.iter().map(canonicalize_value).collect();
            Value::Array(normalized)
        }
        Value::Number(num) => {
            if let Some(f) = num.as_f64() {
                if num.is_i64() || num.is_u64() {
                    Value::Number(num.clone())
                } else {
                    let rounded = round_4(f);
                    if let Some(n) = Number::from_f64(rounded) {
                        Value::Number(n)
                    } else {
                        Value::Number(num.clone())
                    }
                }
            } else {
                Value::Number(num.clone())
            }
        }
        other => other.clone(),
    }
}

/// Converts a serializable value into its canonical JSON string:
/// sorted keys, 4-decimal rounded floats, 2 spaces indentation, and trailing newline.
pub fn to_canonical_json<T: Serialize>(val: &T) -> Result<String> {
    let raw_value =
        serde_json::to_value(val).map_err(|e| CoreError::Serialization(e.to_string()))?;
    let canonical_val = canonicalize_value(&raw_value);

    // Format with 2-space indentation
    let mut buf = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"  ");
    let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
    canonical_val
        .serialize(&mut ser)
        .map_err(|e| CoreError::Serialization(e.to_string()))?;

    // Trailing newline
    buf.push(b'\n');

    String::from_utf8(buf).map_err(|e| CoreError::Serialization(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_canonical_json_sorting_and_rounding() {
        let val = json!({
            "z": 1.234567,
            "a": 10,
            "m": {
                "b": 0.00001,
                "a": -0.0
            }
        });

        let canon_str = to_canonical_json(&val).unwrap();
        let expected = "{\n  \"a\": 10,\n  \"m\": {\n    \"a\": 0.0,\n    \"b\": 0.0\n  },\n  \"z\": 1.2346\n}\n";
        assert_eq!(canon_str, expected);
    }

    #[test]
    fn test_canonical_json_idempotence() {
        let val = json!({
            "b": [std::f64::consts::PI, std::f64::consts::E],
            "a": { "key": "value" }
        });

        let pass1 = to_canonical_json(&val).unwrap();
        let de_pass1: Value = serde_json::from_str(&pass1).unwrap();
        let pass2 = to_canonical_json(&de_pass1).unwrap();

        assert_eq!(pass1, pass2);
    }
}
