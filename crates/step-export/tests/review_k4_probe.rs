//! REVIEW PROBE (review/kernel, K4): every committed KERNEL_VOLUME_*
//! literal round-trips bit-exactly: parse -> fmt_real -> byte-equal,
//! and the reprint parses to identical bits.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use crate::common;

#[test]
fn k4_sidecar_volume_literals_round_trip_bit_exact() {
    let mut n = 0;
    for (name, _) in common::fixture_corpus() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures")
            .join(format!("{name}.expect"));
        let text = std::fs::read_to_string(&path).unwrap();
        for key in ["KERNEL_VOLUME_MM3", "KERNEL_VOLUME_PAD_MM3"] {
            let lit = text
                .lines()
                .find_map(|l| l.strip_prefix(&format!("{key}=")))
                .unwrap_or_else(|| panic!("{name}: {key} missing"));
            let v: f64 = lit.parse().unwrap();
            let printed = step_export::fmt_real(v, "probe").unwrap();
            assert_eq!(
                printed, lit,
                "{name} {key}: printer output != committed literal"
            );
            let back: f64 = printed.parse().unwrap();
            assert_eq!(back.to_bits(), v.to_bits(), "{name} {key}: reparse bits");
            n += 1;
        }
    }
    assert_eq!(n, 34, "17 fixtures x 2 literals");
}
