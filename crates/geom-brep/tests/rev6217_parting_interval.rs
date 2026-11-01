//! rev6217 probe: `tied_germ_turn` before PR 6217 differenced the bends
//! first (`across * (b1 - b0)`, through `enters_material_order2` at
//! speed 1); after it, `parting_side((across * b1, across * b0))`
//! differences after scaling. At `f64` these agree to rounding; at
//! `Interval` a wide `across` no longer cancels, so the new margin is
//! wider exactly where the bends nearly agree.
use geom_brep::{ReferenceNormal, enters_material_order2, parting_side};
use geom_core::{Band, Interval, Tol, Vec3};

fn iv(x: f64, w: f64) -> Interval {
    Interval::from_bounds(x - w, x + w)
}

#[test]
#[ignore = "rev6217 probe: red by design, prints the readings PR 6217 moves"]
fn rev6217_tied_germ_turn_old_vs_new_at_interval() {
    let band = Band::linear(Tol::witness()).unwrap();
    let w = 1e-10;
    let normal = Vec3::new(iv(0.0, w), iv(0.0, w), iv(1.0, w));
    let t = Vec3::new(iv(1.0, w), iv(0.0, w), iv(0.0, w));
    let across = normal.cross(t);
    let r = ReferenceNormal::of_germ(normal, t);
    let mut moved = Vec::new();
    for &big in &[1.0, 10.0, 100.0, 1e3] {
        for &gap in &[1e-9, 1e-6, 1e-3, 1.0] {
            let (b0, b1) = (
                Interval::from_bounds(big, big),
                Interval::from_bounds(big + gap, big + gap),
            );
            let arm = Interval::from_bounds(0.1, 0.1);
            let old = enters_material_order2(
                across * (b1 - b0),
                Interval::from_bounds(1.0, 1.0),
                r,
                arm,
                band,
            );
            let new = parting_side((across * b1, across * b0), r, arm, band);
            if format!("{old:?}") != format!("{new:?}") {
                moved.push(format!("bends {big:e}+{gap:e}: old {old:?} new {new:?}"));
            }
        }
    }
    for m in &moved {
        eprintln!("{m}");
    }
    assert!(moved.is_empty(), "{} readings moved", moved.len());
}
