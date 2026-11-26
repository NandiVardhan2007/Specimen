use proptest::prelude::*;
use specimen_core::{canonicalize_value, round_4, round_f64, ItemId, ItemType, RelPath};

proptest! {
    #[test]
    fn prop_rounding_idempotent(val in any::<f64>()) {
        if !val.is_nan() && !val.is_infinite() {
            let r1 = round_4(val);
            let r2 = round_4(r1);
            prop_assert_eq!(r1, r2);
        }
    }

    #[test]
    fn prop_rounding_normalized_zero(val in -0.00000000001f64..0.00000000001f64) {
        let r = round_f64(val, 4);
        prop_assert_eq!(r, 0.0);
        prop_assert!(!r.is_sign_negative() || r == 0.0);
    }

    #[test]
    fn prop_canonical_json_idempotence(
        val_f64 in any::<f64>(),
        val_i64 in any::<i64>(),
        val_str in "[a-zA-Z0-9_-]{0,20}"
    ) {
        if !val_f64.is_nan() && !val_f64.is_infinite() {
            let json_val = serde_json::json!({
                "b": val_f64,
                "a": val_i64,
                "s": val_str,
            });
            let c1 = canonicalize_value(&json_val);
            let c2 = canonicalize_value(&c1);
            prop_assert_eq!(c1, c2);
        }
    }

    #[test]
    fn prop_item_id_determinism(
        slug in "[a-z0-9-]{1,20}",
        data in "[a-zA-Z0-9 ]{1,50}"
    ) {
        let id1 = ItemId::generate(ItemType::Icon, &slug, &data);
        let id2 = ItemId::generate(ItemType::Icon, &slug, &data);
        prop_assert_eq!(&id1, &id2);
        prop_assert_eq!(id1.item_type(), ItemType::Icon);
        prop_assert_eq!(id1.hash8().len(), 8);
    }

    #[test]
    fn prop_rel_path_safe_segments(
        seg1 in "[a-zA-Z0-9_-]{1,10}",
        seg2 in "[a-zA-Z0-9_-]{1,10}"
    ) {
        let path_str = format!("{}/{}", seg1, seg2);
        let rel_path = RelPath::new(&path_str).unwrap();
        prop_assert_eq!(rel_path.as_str(), &path_str);
    }
}
