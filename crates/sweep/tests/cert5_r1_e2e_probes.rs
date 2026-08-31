//! CERT-5 review lane R1 end-to-end probe (blinded adversarial review
//! of PR 1314, frozen head 3fc450d6). NOT for merge.
//!
//! The reviewer's OWN body — not the unit's blades, not dm1: a
//! profile with a SIXTY-degree arc (single rational sub-arc, weight
//! cos 30 deg — no unit fixture's), lofted at six stations on a
//! quadratic skin: off-grid interior V knots at 0.3 and 0.7, driven
//! through the PUBLIC props door.
//!
//! MEASURED LIMITS, found while probing (all pre-existing — the PR
//! touches none of the loft/pcurve code):
//! - a 3-sub-arc profile arc (>180 deg, the only native route to
//!   OFF-GRID interior u knots) never lofts: build refuses at pcurve
//!   seam certification ("the seam carrier is not the chart's own
//!   boundary row");
//! - the same refusal fires for identical-section stacks at
//!   (stations, degree, size, height) = (7,2,1,2), (8,3,1,2),
//!   (9,3,1,2), (6,2,0.6,1.75), and even the unit's own blade
//!   profile at height 1.75 instead of 2.0 — the buildable family
//!   is a narrow pocket around the unit's fixtures, and the
//!   quadrature is unreachable outside it.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use geom_core::{Affine3, Point2, Vec3};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::{Section, loft_body};

/// tan(150/4 deg): one 150-degree bulge arc (two rational sub-arcs).
const BULGE_270: f64 = 0.267_949_192_431_122_7;

/// The balloon: a flat chord of two straight segments closed by a
/// 270-degree arc bulging up. Simple (the arc's lowest points are its
/// endpoints, ON the chord), counterclockwise.
fn balloon_section() -> Section {
    let v = |x: f64, y: f64, bulge: f64| ProfileVertex::new(Point2::new(x, y), bulge);
    vec![ProfileLoop::new(vec![
        v(-1.0, -1.0, 0.0),
        v(1.0, -1.0, BULGE_270),
        v(1.0, 1.0, 0.0),
        v(-1.0, 1.0, 0.0),
    ])]
}

const HEIGHT: f64 = 2.0;
const STATIONS: usize = 6;
const V_DEGREE: usize = 2;

#[test]
fn own_body_offgrid_knots_both_directions_certifies_and_contains_oracle() {
    let sections: Vec<Section> = (0..STATIONS).map(|_| balloon_section()).collect();
    #[allow(clippy::cast_precision_loss)]
    let places: Vec<Affine3<f64>> = (0..STATIONS)
        .map(|k| {
            Affine3::translation(Vec3::new(
                0.0,
                0.0,
                HEIGHT * k as f64 / (STATIONS - 1) as f64,
            ))
        })
        .collect();
    let lofted =
        loft_body::<f64>(&sections, &places, V_DEGREE, Tol::witness()).expect("the balloon lofts");

    // The V-direction hypothesis, asserted on the fixture: interior
    // knots are running means of V_DEGREE consecutive section
    // parameters, and at 7 even stations two of the four are off
    // every dyadic grid the composite cuts (5/12, 7/12).
    let params = &lofted.section_params;
    #[allow(clippy::cast_precision_loss)]
    let interior: Vec<f64> = (1..params.len() - V_DEGREE)
        .map(|j| params[j..j + V_DEGREE].iter().sum::<f64>() / V_DEGREE as f64)
        .collect();
    let off = interior
        .iter()
        .filter(|k| {
            let mut pieces = 8u32;
            let mut on = false;
            while pieces <= 1024 {
                let s = *k * f64::from(pieces);
                if (s - s.round()).abs() < 1e-12 {
                    on = true;
                }
                pieces *= 2;
            }
            !on
        })
        .count();
    eprintln!("CERT5-R1 balloon: interior v knots {interior:?}, off-grid {off}");
    assert!(
        off >= 2,
        "the balloon must carry off-grid interior v knots (got {off})"
    );

    // The oracle: the same profile EXTRUDED — the arc wall is an
    // analytic cylinder, closed form, pad exactly 0.
    let prof = Profile::new(SketchPlane::xy(), balloon_section())
        .validate(Tol::witness())
        .expect("the balloon profile validates");
    let oracle = sweep::extrude::<f64>(&prof, sweep::Extrusion::Distance(HEIGHT), Tol::witness())
        .expect("extrude");
    let want = topo::mass_properties(&oracle.body, Tol::witness()).expect("analytic oracle");
    assert_eq!(want.volume_pad, 0.0, "the oracle must be a closed form");

    topo::validate_closed(&lofted.body).expect("tiers 1/2 admit the balloon");
    let t0 = std::time::Instant::now();
    let got = topo::mass_properties(&lofted.body, Tol::witness());
    let dt = t0.elapsed();
    match got {
        Ok(m) => {
            eprintln!(
                "CERT5-R1 balloon: certified volume {} +- {} in {dt:?}; oracle {}",
                m.volume, m.volume_pad, want.volume
            );
            assert!(
                (m.volume - want.volume).abs() <= m.volume_pad,
                "E2E ACCURACY: the enclosure must CONTAIN the analytic volume: \
                 got {} +- {}, oracle {}",
                m.volume,
                m.volume_pad,
                want.volume
            );
            // The retired-floor ceiling, same shape as the unit's own
            // rows: nowhere near the straddle floor.
            assert!(
                m.volume_pad < 1.0e-5,
                "the pad must sit under the retired-floor ceiling: {}",
                m.volume_pad
            );
        }
        Err(e) => panic!(
            "the balloon must certify at the default eps through the public door \
             (off-grid knots in both directions are exactly the retired defect): {e}"
        ),
    }
}
