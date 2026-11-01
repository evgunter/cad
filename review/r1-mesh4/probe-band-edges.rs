// R1 probe (transient): appended to crates/mesh/src/sizing.rs's test
// module, run, then reverted — the byte-identity of the PR-owned file
// is re-verified after. Kept here as the record of what ran.
//
// Attack: claim 2/3 — each operation computes EXACTLY the boolean the
// bare spelling computed, checked at the representable neighbours of
// the band edge and on the non-ordered inputs, by direct parity with
// the bare comparison over a value sweep.

#[test]
fn r1_probe_ops_are_bitwise_the_bare_spellings() {
    let band = 1e-9f64;
    let e = Eps::exactly(band);
    let up = f64::from_bits(band.to_bits() + 1); // next_up(band)
    let down = f64::from_bits(band.to_bits() - 1); // next_down(band)
    let sweep = [
        0.0,
        -0.0,
        f64::MIN_POSITIVE,
        down,
        band,
        up,
        2e-9,
        1e-6,
        1.0,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::NAN,
        -1.0,
    ];
    for x in sweep {
        assert_eq!(e.separates(x), x > band, "separates parity at {x:?}");
        assert_eq!(e.coincident(x), x <= band, "coincident parity at {x:?}");
        assert_eq!(e.dominates(x), x < band, "dominates parity at {x:?}");
        let padded = e.pad(x);
        let bare = x + band;
        assert_eq!(
            padded.to_bits(),
            bare.to_bits(),
            "pad parity at {x:?} (bitwise, NaN included)"
        );
    }
    // The three predicates at the exact representable neighbours:
    assert!(e.coincident(down) && e.dominates(down) && !e.separates(down));
    assert!(e.coincident(band) && !e.dominates(band) && !e.separates(band));
    assert!(!e.coincident(up) && !e.dominates(up) && e.separates(up));
}
