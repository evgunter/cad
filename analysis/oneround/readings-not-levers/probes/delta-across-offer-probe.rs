// Delta-read probe (appended to crates/geom-core/src/k_stats.rs at cb36266dd, run with
// `cargo test -p geom-core --lib review_probe_across -- --nocapture`).
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod review_probe_across_offer {
    use super::decide_across;
    use crate::{Band, Interval, Sign};

    #[test]
    fn review_probe_none_offered_where_a_tolerance_decides() {
        let names = ["row", "row_floor"];
        let band = Band::new(1e-9, 1e-8).unwrap();
        for (datum, bands) in [(3e-10, 9.9), (5e-9, 9.9), (1e-9, 1.5), (2e-10, 0.4)] {
            let r = decide_across(names, datum, bands * band.zero(), band);
            let offers = r.as_ref().err().map(|e| e.offers_tolerance());
            let mut decided_at = None;
            let mut z = band.zero();
            for _ in 0..80 {
                z *= 0.7;
                let b = Band::new(z, 10.0 * z).unwrap();
                if decide_across(names, datum, bands * z, b) == Ok(Sign::Positive) {
                    decided_at = Some(z);
                    break;
                }
            }
            println!("datum {datum:e} swing {bands} bands: at run band {:?} offers {offers:?}; decided Positive at eps {decided_at:?}", r.as_ref().map_err(|e| e.predicate));
            let ri = decide_across(names, Interval::from_bounds(datum, datum), Interval::from_bounds(bands*band.zero(), bands*band.zero()), band);
            println!("   interval: offers {:?}", ri.err().map(|e| e.offers_tolerance()));
        }
    }
}
// Output at cb36266dd:
// datum 3e-10 swing 9.9 bands: at run band Err(Some("row_floor")) offers Some(false); decided Positive at eps Some(1.384128720099999e-11)
// datum 5e-9 swing 9.9 bands: at run band Err(Some("row_floor")) offers Some(false); decided Positive at eps Some(2.401e-10)
// datum 1e-9 swing 1.5 bands: at run band Err(Some("row")) offers Some(false); decided Positive at eps Some(8.235429999999997e-11)
// datum 2e-10 swing 0.4 bands: at run band Ok(Zero) offers None; decided Positive at eps Some(1.384128720099999e-11)
// (Interval lane: the same offers.)
