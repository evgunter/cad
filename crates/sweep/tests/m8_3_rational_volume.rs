//! **M8-3 — rational walls are VOLUME-COMPUTABLE.**
//!
//! The soundness pins for the rational patch-flux enclosure, in the
//! SKINFIT two-assertion shape (`m7_skin_integral.rs`): pin the
//! accuracy against an INDEPENDENT oracle **and separately** pin the
//! pad ceiling, so a loosening enclosure can never quietly absorb the
//! tolerance.
//!
//! The oracle is the strongest available: the *same solid* built by
//! `extrude`, whose bulged wall is an analytic `Surface::Cylinder`
//! closed form with pad exactly 0 — a different surface
//! representation, a different props lane, and no shared arithmetic
//! with the quadrature under test.

use geom_core::{Affine3, Point2, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::{Section, loft_body};

/// A unit square with a quarter-circle bulge on the `+x` side — the
/// arc-bearing profile whose lofted wall is RATIONAL (weights
/// `1, cos 22.5°, 1` over two 45° sub-arcs).
fn arc_section(s: f64) -> Section {
    let v = |x: f64, y: f64, bulge: f64| ProfileVertex {
        pos: Point2::new(x, y),
        bulge,
    };
    vec![ProfileLoop::new(vec![
        v(-s, -s, 0.0),
        // tan(π/8): a quarter-circle bulge-out.
        v(s, -s, 0.4142135623730951),
        v(s, s, 0.0),
        v(-s, s, 0.0),
    ])]
}

fn stack(z: [f64; 3]) -> Vec<Affine3<f64>> {
    z.map(|h| Affine3::translation(Vec3::new(0.0, 0.0, h)))
        .into()
}

/// **The arc PRISM** (the `#288` waypoint's body): three identical
/// arc sections stacked, so the loft reproduces an extrusion exactly.
///
/// Assertion 1 — ACCURACY against the independent analytic oracle.
/// Assertion 2 — the PAD CEILING, pinned separately.
#[test]
fn arc_prism_volume_brackets_the_analytic_extrusion() {
    let loft = loft_body::<f64>(
        &[arc_section(1.0), arc_section(1.0), arc_section(1.0)],
        &stack([0.0, 1.0, 2.0]),
        2,
    )
    .expect("the arc prism lofts")
    .body;
    topo::validate_geometric(&loft).expect("tier 3 certifies a rational-wall body (M8-3 flip)");
    let got = topo::mass_properties(&loft).expect("the rational wall is volume-computable");

    // The oracle: the same solid through `extrude`, whose bulged wall
    // is an analytic cylinder (closed form, pad 0).
    let prof = Profile::new(SketchPlane::xy(), arc_section(1.0))
        .validate(geom_core::Tolerance::get())
        .expect("the profile validates");
    let oracle = sweep::extrude::<f64>(&prof, sweep::Extrusion::Distance(2.0)).expect("extrude");
    let want = topo::mass_properties(&oracle.body).expect("analytic mass properties");
    assert_eq!(want.volume_pad, 0.0, "the oracle must be a closed form");

    // 1. ACCURACY: the certified enclosure contains the oracle.
    assert!(
        (got.volume - want.volume).abs() <= got.volume_pad,
        "the rational enclosure must CONTAIN the analytic volume: \
         got {} ± {}, oracle {}",
        got.volume,
        got.volume_pad,
        want.volume,
    );
    // 2. PAD CEILING, pinned separately: a loosening enclosure cannot
    // absorb assertion 1 without tripping this.
    assert!(
        got.volume_pad < 5.0e-6,
        "volume pad ceiling: {} (M8-3 measured 9.82e-7)",
        got.volume_pad,
    );
}

/// **The arc LOFT** (`#276`'s honestly-refused class): sections of
/// DIFFERENT scale, so the rational wall genuinely varies in `v` and
/// no analytic body reproduces it. The oracle here is the enclosure's
/// own internal consistency plus the pad ceiling; the accuracy oracle
/// is the prism row above, which shares every line of the lane.
#[test]
fn arc_loft_is_volume_computable_with_a_pinned_pad() {
    let loft = loft_body::<f64>(
        &[arc_section(1.0), arc_section(1.25), arc_section(1.0)],
        &stack([0.0, 1.0, 2.0]),
        2,
    )
    .expect("the arc loft builds")
    .body;
    topo::validate_geometric(&loft).expect("tier 3 certifies the arc loft (M8-3 flip of #276)");
    let got = topo::mass_properties(&loft).expect("the rational wall is volume-computable");
    // A loft that bulges outward in the middle must exceed the prism.
    assert!(
        got.volume > 12.0 && got.volume < 13.0,
        "arc-loft volume out of band: {}",
        got.volume,
    );
    assert!(
        got.volume_pad < 5.0e-6,
        "volume pad ceiling: {} (M8-3 measured 1.01e-6)",
        got.volume_pad,
    );
}
