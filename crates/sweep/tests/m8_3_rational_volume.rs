//! **M8-3 — rational walls become VOLUME-COMPUTABLE.**
//!
//! The soundness pins for the rational patch-flux enclosure reached
//! from a BODY (the arc-rim chart map is what makes the body
//! assemble), in the SKINFIT two-assertion shape
//! (`m7_skin_integral.rs`): pin the accuracy against an INDEPENDENT
//! oracle **and separately** pin the pad ceiling, so a loosening
//! enclosure can never quietly absorb the tolerance.
//!
//! The oracle is the strongest available: the *same solid* built by
//! `extrude`, whose bulged wall is an analytic `Surface::Cylinder`
//! closed form with pad exactly 0 — a different surface
//! representation, a different props lane, and no shared arithmetic
//! with the quadrature under test.
//!
//! # ε posture (the PR-1 discipline, applied at the BODY level)
//!
//! The convergence target is `1024·ε` against a **fixed** schedule
//! (D9), so a body that certifies at one ε honestly may not at a
//! tighter one. These rows therefore pin all three honest outcomes
//! and never widen a target: `Certified` (the enclosure must CONTAIN
//! the oracle and respect its pad ceiling), `Budget` (a typed
//! `QuadratureBudget` whose width really missed a target that really
//! is `1024·ε`), `Escalated` (only `props_quad_converged` may
//! escalate). Anything else panics.

use geom_brep::PropsError;
use geom_core::{Affine3, Point2, Tolerance, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::{Section, loft_body};
use topo::{MassProperties, MassPropsError};

/// The convergence target's tolerance factor, mirrored from
/// `geom_brep::props::quad` (private there; the pin is that the
/// refusal's own `target_len` equals this times the run's ε).
const QUAD_TARGET_LEN_FACTOR: f64 = 1024.0;

/// The three honest outcomes of a certified quadrature at a given ε.
#[derive(Debug, PartialEq, Eq)]
enum EpsPosture {
    /// Bounds returned; the caller pins accuracy and the pad ceiling.
    Certified,
    /// The fixed schedule ran out before `1024·ε` — typed, with the
    /// measured width.
    Budget,
    /// The convergence predicate could not be decided in-band.
    Escalated,
}

/// Classify a body-level mass-properties outcome, asserting each
/// posture's own invariants. Anything outside the three is a real
/// failure and panics.
fn body_posture(row: &str, out: &Result<MassProperties<f64>, MassPropsError>) -> EpsPosture {
    let target = QUAD_TARGET_LEN_FACTOR * Tolerance::get().eps;
    match out {
        Ok(_) => EpsPosture::Certified,
        Err(MassPropsError::Face {
            source:
                PropsError::QuadratureBudget {
                    width_len,
                    target_len,
                },
            ..
        }) => {
            assert!(
                width_len.is_finite() && width_len > target_len,
                "{row}: a budget refusal must carry a width that really missed: \
                 {width_len:e} vs {target_len:e}"
            );
            assert!(
                (target_len - target).abs() <= target * 1e-12,
                "{row}: the refused target must BE 1024·ε for this run: \
                 {target_len:e} vs {target:e}"
            );
            EpsPosture::Budget
        }
        Err(MassPropsError::Face {
            source: PropsError::Escalated { cause },
            ..
        }) => {
            assert_eq!(
                cause.predicate,
                Some("props_quad_converged"),
                "{row}: only the convergence predicate may escalate here: {cause:?}"
            );
            EpsPosture::Escalated
        }
        Err(other) => panic!("{row}: not an honest quadrature posture: {other}"),
    }
}

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
/// The `#288` claim this row settles is the CONSTRUCTION one — the
/// body assembles, its rational wall's arc rims mint and certify, and
/// tier 3 admits it. Whether the volume itself certifies is the ε
/// question above, pinned per posture.
#[test]
fn arc_prism_volume_brackets_the_analytic_extrusion() {
    let loft = loft_body::<f64>(
        &[arc_section(1.0), arc_section(1.0), arc_section(1.0)],
        &stack([0.0, 1.0, 2.0]),
        2,
    )
    .expect("the arc prism lofts")
    .body;
    let got = topo::mass_properties(&loft);
    let posture = body_posture("arc prism", &got);
    eprintln!(
        "EPS-ROW arc prism @ eps={:e}: {posture:?}{}",
        Tolerance::get().eps,
        match &got {
            Ok(m) => format!(" volume {} ± {}", m.volume, m.volume_pad),
            Err(e) => format!(" ({e})"),
        }
    );
    if posture != EpsPosture::Certified {
        // The honest non-certifying postures still say the CHART map
        // landed: a body whose arc rims did not mint would have failed
        // to build, long before any quadrature ran.
        return;
    }
    let got = got.expect("certified");

    // The oracle: the same solid through `extrude`, whose bulged wall
    // is an analytic cylinder (closed form, pad 0).
    let prof = Profile::new(SketchPlane::xy(), arc_section(1.0))
        .validate(Tolerance::get())
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
    // absorb assertion 1 without tripping this. The ceiling is keyed
    // to ε because the schedule is fixed and the pad is what the
    // schedule achieves — never widened past the run's own target.
    let ceiling = 2.0 * QUAD_TARGET_LEN_FACTOR * Tolerance::get().eps;
    assert!(
        got.volume_pad < ceiling,
        "volume pad ceiling: {} vs {ceiling} (M8-3 measured {} at ε=1e-9)",
        got.volume_pad,
        ARC_PRISM_PAD_AT_DEFAULT_EPS,
    );
}

/// The pad measured on the arc prism at the default ε (1e-9), on the
/// POST-#313 hulls (the shared midpoint-plus-Taylor area rule). Kept
/// as a named number so the ceiling assertion above can say what the
/// lane actually achieves, not only what it must beat.
///
/// It is the SAME number the pre-#313 hulls gave, and that is not a
/// coincidence: `volume_pad` is flux-side, and #313 healed the AREA
/// rule. The area pad moved; the volume pad could not.
const ARC_PRISM_PAD_AT_DEFAULT_EPS: f64 = 9.816_714_3e-7;

/// The same for the arc loft.
const ARC_LOFT_PAD_AT_DEFAULT_EPS: f64 = 1.009_875_4e-6;

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
    let got = topo::mass_properties(&loft);
    let posture = body_posture("arc loft", &got);
    eprintln!(
        "EPS-ROW arc loft @ eps={:e}: {posture:?}{}",
        Tolerance::get().eps,
        match &got {
            Ok(m) => format!(" volume {} ± {}", m.volume, m.volume_pad),
            Err(e) => format!(" ({e})"),
        }
    );
    if posture != EpsPosture::Certified {
        return;
    }
    let got = got.expect("certified");
    // A loft that bulges outward in the middle must exceed the prism.
    assert!(
        got.volume > 12.0 && got.volume < 13.0,
        "arc-loft volume out of band: {}",
        got.volume,
    );
    let ceiling = 2.0 * QUAD_TARGET_LEN_FACTOR * Tolerance::get().eps;
    assert!(
        got.volume_pad < ceiling,
        "volume pad ceiling: {} vs {ceiling} (M8-3 measured {} at ε=1e-9)",
        got.volume_pad,
        ARC_LOFT_PAD_AT_DEFAULT_EPS,
    );
}

/// **Tier 3 admits a rational-wall body** — the `#288`/`#276`
/// construction flip, stated on its own so it is pinned even at an ε
/// where the volume lands in the budget posture. Tier 3 runs the +V
/// invariant, which consumes the quadrature; a budget refusal there
/// is an honest tier-3 refusal, so this row pins the CHART map
/// (rims mint and certify) at every ε and the tier-3 verdict only
/// where the quadrature certifies.
#[test]
fn tier3_admits_the_rational_wall_body() {
    let loft = loft_body::<f64>(
        &[arc_section(1.0), arc_section(1.0), arc_section(1.0)],
        &stack([0.0, 1.0, 2.0]),
        2,
    )
    .expect("the arc prism lofts (the pcurve mint is inside the build)")
    .body;
    // Structural tiers never touch quadrature: the body IS closed and
    // its rational-wall pcurves are minted and certified.
    topo::validate_closed(&loft).expect("tier 1/2 admit the rational-wall body");
    let out = topo::mass_properties(&loft);
    if body_posture("arc prism (tier 3)", &out) == EpsPosture::Certified {
        topo::validate_geometric(&loft)
            .expect("tier 3 certifies a rational-wall body (M8-3 flip of #288/#276)");
    }
}
