//! M5 PR 11 §2 acceptance: certified mass properties for curved-CUT
//! faces — the tilted-cut halves (shape (i)) get certified volume/area
//! ENCLOSURES from the quadrature lane, tier 3's check 7 flips from
//! `VolumeUncomputable` to green, and the brackets contain the closed
//! forms.
//!
//! Closed forms (r = 0.5, H = 1, tilt φ = 0.3 through the mid-height
//! axis point): each half's volume is exactly `πr²H/2` (the plane
//! passes through the axis midpoint, so the wedge above cancels the
//! wedge below); the below-half's area is
//! `πr² + πr·H + πr²/cos φ` (cap + wall — the sinusoid's mean height
//! is H/2, twice — + ellipse section πab = πr²/cos φ).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use geom_core::{Point2, Point3, Vec3};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane, ValidatedProfile};
use sweep::{Extrusion, extrude};
use topo::splitting::{SplitPart, SplitPlane, split};
use topo::{Body, validate_geometric};

const R: f64 = 0.5;
const H: f64 = 1.0;
const PHI: f64 = 0.3;

fn disc() -> ValidatedProfile<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(Point2::new(-R, 0.0), 1.0),
        ProfileVertex::new(Point2::new(R, 0.0), 1.0),
    ]);
    Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap()
}

fn halves() -> (Body<f64>, Body<f64>) {
    let cylinder = extrude(&disc(), Extrusion::Distance(H), Tol::witness())
        .unwrap()
        .body;
    let plane = SplitPlane {
        origin: Point3::new(0.0, 0.0, H / 2.0),
        normal: Vec3::new(PHI.sin(), 0.0, PHI.cos()),
    };
    let result =
        split(&cylinder, &plane, Tol::witness()).expect("the tilted cut splits the cylinder");
    let (SplitPart::Body(above), SplitPart::Body(below)) = (&result.above, &result.below) else {
        panic!("both sides carry material");
    };
    (above.clone(), below.clone())
}

/// The quadrature lane computes; the enclosure brackets the closed
/// form; the pads are honest (nonzero on the cut walls) and small.
#[test]
fn tilted_halves_volume_enclosure_brackets_the_closed_form() {
    let (above, below) = halves();
    let half_exact = core::f64::consts::PI * R * R * H / 2.0;
    for (label, body) in [("above", &above), ("below", &below)] {
        let m = topo::mass_properties(body, Tol::witness())
            .unwrap_or_else(|e| panic!("{label}: the PR 11 quadrature lane computes: {e:?}"));
        assert!(
            m.volume_pad > 0.0,
            "{label}: a curved-cut half carries a certified (nonzero) volume pad"
        );
        assert!(
            m.volume - m.volume_pad <= half_exact && half_exact <= m.volume + m.volume_pad,
            "{label}: certified bracket [{}, {}] must contain πr²H/2 = {half_exact}",
            m.volume - m.volume_pad,
            m.volume + m.volume_pad
        );
        // The bracket is also USEFUL: half-widths far below the value.
        assert!(
            m.volume_pad < 1e-3 * half_exact,
            "{label}: pad {} vs volume {half_exact}",
            m.volume_pad
        );
    }
}

/// The area enclosure brackets the closed form, on BOTH halves — the
/// two areas are equal (each wall's mean height is H/2), so one closed
/// form covers the pair, and the halves' pads differ by up to 1.5x.
#[test]
fn half_area_enclosures_bracket_the_closed_form() {
    let (above, below) = halves();
    let pi = core::f64::consts::PI;
    let exact = pi * R * R + pi * R * H + pi * R * R / PHI.cos();
    for (label, body) in [("above", &above), ("below", &below)] {
        let m = topo::mass_properties(body, Tol::witness()).unwrap();
        assert!(
            m.area_pad > 0.0,
            "{label}: the cut wall's area is a certified enclosure"
        );
        assert!(
            m.surface_area - m.area_pad <= exact && exact <= m.surface_area + m.area_pad,
            "{label}: area bracket [{}, {}] must contain {exact}",
            m.surface_area - m.area_pad,
            m.surface_area + m.area_pad
        );
        // Positivity and containment both get EASIER as a certified
        // width grows, so on their own they report no widening at all.
        // The two rows below are what report one.
        //
        // FIRST, the area bracket is PINNED TO THE FLUX BRACKET — the
        // one the kernel actually meters. On this lane
        // `area = |r·A_s|` and `flux = r²·A_s + o·A⃗` share the one
        // signed UV area `A_s`, and `o·A⃗` is a closed form entering
        // flux alone, so `width(flux) ≥ r²·width(A_s)`. With
        // `area_pad = width(area)/2` and `volume_pad = width(flux)/6`
        // that gives
        //
        //     area_pad ≤ (3/r)·volume_pad
        //
        // as an identity of the arithmetic — `R` below is the
        // fixture's own radius, not a fitted constant. It holds to
        // within 5e-7 relative at every ε, the shortfall being
        // `o·A⃗`'s own width.
        //
        // SCOPE: this is the cylinder-cut arm on a body whose only
        // quadrature faces lie on one cylinder of radius `R` — true
        // here because each half's cap and elliptical section are
        // planar, hence exact and pad-free. It is NOT a property of
        // the patch lanes, whose area has its own fixed resolution and
        // no such tie (there the same body can carry a 1e-13 volume
        // pad against a 0.2 area pad).
        //
        // This is the row that reports the defect as stated: it goes
        // red when the area bracket stops shrinking alongside the flux
        // bracket, not merely when the pad gets large, and it bites
        // identically on every ε leg.
        assert!(
            m.area_pad <= (3.0 / R) * m.volume_pad * (1.0 + 1e-9),
            "{label}: area pad {} exceeds {} = (3/r)·volume_pad",
            m.area_pad,
            (3.0 / R) * m.volume_pad
        );
        // SECOND, an outer backstop, which the pin above does not
        // give: it would stay green if BOTH brackets widened together.
        // The anchor is what this lane can structurally produce. Its
        // width falls monotonically in the piece count (measured over
        // 16..=4096, where the funnel's reachable set is 16·2^k), so
        // the widest enclosure it can return is at the initial count
        // with no refinement round taken: 1.47e-4·exact, over both
        // halves and all three ε legs, and CI's ε = 1e-6 leg already
        // sits there. The ceiling clears that maximum by 2.0x
        // (re-derived: the binding ratio is 1.470023e-4 at the
        // ε = 1e-6 `above` half, so 2.041x). This is the CYLINDER arm,
        // whose area rides `harmonic_edge_integral` rather than the
        // patch lanes' cell rule — which is why every figure here is
        // bit-identical to the one this row was first written against,
        // while the loft row's moved when that cell rule changed.
        assert!(
            m.area_pad < 3e-4 * exact,
            "{label}: area pad {} vs exact area {exact}",
            m.area_pad
        );
    }
}

/// Tier 3 flips: check 7 (VolumeUncomputable) retires for the newly
/// integrable class — the halves are geometrically valid END TO END,
/// with the +V backstop consuming the certified bounds.
#[test]
fn tier3_passes_on_both_halves() {
    let (above, below) = halves();
    for (label, body) in [("above", &above), ("below", &below)] {
        if let Err(errs) = validate_geometric(body, Tol::witness()) {
            panic!("{label}: tier 3 must pass with the quadrature lane live: {errs:?}");
        }
    }
}

/// D9: the quadrature is deterministic — same body, same bits (fixed
/// piece schedules, fixed reduction order, no ambient state).
#[test]
fn quadrature_is_bit_deterministic() {
    let (above, _) = halves();
    let a = topo::mass_properties(&above, Tol::witness()).unwrap();
    let b = topo::mass_properties(&above, Tol::witness()).unwrap();
    assert_eq!(a.volume.to_bits(), b.volume.to_bits());
    assert_eq!(a.surface_area.to_bits(), b.surface_area.to_bits());
    assert_eq!(a.volume_pad.to_bits(), b.volume_pad.to_bits());
    assert_eq!(a.area_pad.to_bits(), b.area_pad.to_bits());
}

/// The two halves complement: volumes sum to the full cylinder well
/// inside the summed certified pads.
#[test]
fn halves_sum_to_the_cylinder() {
    let (above, below) = halves();
    let a = topo::mass_properties(&above, Tol::witness()).unwrap();
    let b = topo::mass_properties(&below, Tol::witness()).unwrap();
    let full = core::f64::consts::PI * R * R * H;
    let sum = a.volume + b.volume;
    let pad = a.volume_pad + b.volume_pad;
    assert!(
        (sum - full).abs() <= pad,
        "sum {sum} vs cylinder {full} (allowed {pad})"
    );
    // Containment alone is monotone the wrong way, and here it is
    // loose by three orders: the residual runs 2.7e-4 to 3.6e-3 of the
    // summed pad across the ε legs, so a complementarity failure of
    // ~270x would still pass. The band is the pads' to set — the two
    // per-half ceilings above bound them — and what this row owes is
    // that the halves agree far better than their pads require.
    assert!(
        (sum - full).abs() <= 0.05 * pad,
        "the halves' residual {} is not small against the summed pad {pad}",
        (sum - full).abs()
    );
}

/// The LANE SPLIT (Evan's PR 11 ruling): the dual lane instantiates
/// none of the certified quadrature — a trimmed face at `Dual64`
/// keeps the closed form's typed refusal (volume certification is the
/// CERTIFYING lanes' business; derivative transport is the dual's — a
/// dual carries a bracket since D1, 2026-08-19, and still may not
/// certify).
#[test]
fn dual_lane_keeps_the_closed_form_refusal() {
    use geom_core::Dual64;
    let d = |x: f64| Dual64::constant(x);
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(Point2::new(d(-R), d(0.0)), d(1.0)),
        ProfileVertex::new(Point2::new(d(R), d(0.0)), d(1.0)),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let cylinder = extrude(&profile, Extrusion::Distance(d(H)), Tol::witness())
        .unwrap()
        .body;
    let plane = SplitPlane {
        origin: Point3::new(d(0.0), d(0.0), d(H / 2.0)),
        normal: Vec3::new(d(PHI.sin()), d(0.0), d(PHI.cos())),
    };
    let result = split(&cylinder, &plane, Tol::witness()).unwrap();
    let SplitPart::Body(above) = &result.above else {
        panic!("above carries material");
    };
    match topo::mass_properties(above, Tol::witness()) {
        Err(topo::MassPropsError::Face { source, .. }) => {
            let msg = format!("{source}");
            assert!(msg.contains("ellipse arc"), "{msg}");
        }
        other => panic!("the dual lane keeps the typed closed-form refusal, got {other:?}"),
    }
}
