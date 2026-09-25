//! **A cap ring beside a convex ruled crease's cut-off.** The ruled band
//! removes, from each transverse cap, the SLIVER between the cut-off
//! arc (radius `r` about the spine's crossing `c`) and the crease's old
//! vertex `V`. A ring of the cap lying in that sliver would be left on
//! the cap outside the region it bounds, so every other cycle of the
//! cap is metered against the sliver's enclosing annulus
//! `r ≤ ‖p − c‖ ≤ ρ` before any carve (`ρ` the farthest the sliver
//! reaches from `c`, which on this rod is `‖V − c‖`).
//!
//! The fixture is the D-rod (`test_support::rod_d_profile_at`'s D at
//! [`ROD_FLAT`]) with a second profile loop: a round bore through it.
//! Its upper crease's ball centre is `c = (0.2, √0.12)` and its old
//! vertex `V = (0.3, 0.4)`, so at [`ROD_FILLET`] the annulus is
//! `0.1 ≤ ‖p − c‖ ≤ ‖V − c‖ ≈ 0.1134`.
//!
//! The keyhole twin — a bore beside a cut-off that runs in a cap RING —
//! is `review_band_ruled_ring_probes`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Sign, Tol};
use profile::{ProfileLoop, SketchPlane, test_support::bulge_loop};
use sweep::blend::{BlendError, fillet_edges};
use sweep::test_support::{
    ROD_FILLET, ROD_FLAT, ROD_L, ROD_R, extruded, rod_chord_at, rod_creases, rod_section_cut,
};
use topo::{Body, mass_properties, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

fn volume(body: &Body<f64>) -> f64 {
    mass_properties(body, tol())
        .expect("closed-form props")
        .volume
}

/// A round profile loop: two semicircles about `(x, y)`.
fn bore(x: f64, y: f64, a: f64) -> ProfileLoop<f64> {
    bulge_loop(vec![
        (Point2::new(x + a, y), 1.0),
        (Point2::new(x - a, y), 1.0),
    ])
}

/// The D-rod with one round bore through it.
fn bored_d_rod(x: f64, y: f64, a: f64) -> Body<f64> {
    let c = rod_chord_at(ROD_FLAT);
    let d = bulge_loop(vec![
        (Point2::new(ROD_FLAT, c.half), c.wall_bulge),
        (Point2::new(ROD_FLAT, -c.half), 0.0),
    ]);
    extruded(SketchPlane::xy(), vec![d, bore(x, y, a)], ROD_L, tol())
}

/// The upper crease's ball centre `c` and old vertex `V` in the cap.
fn upper_corner(r: f64) -> ((f64, f64), (f64, f64)) {
    let cx = ROD_FLAT - r;
    let c = (cx, ((ROD_R - r).powi(2) - cx * cx).sqrt());
    let v = (ROD_FLAT, (ROD_R.powi(2) - ROD_FLAT.powi(2)).sqrt());
    (c, v)
}

/// The refusal a bore in (or across) the sliver owes: `RingClearance`
/// at a cap face, definitely negative.
fn assert_cap_ring_refusal(body: &Body<f64>, what: &str) {
    validate_geometric(body, tol()).unwrap_or_else(|e| panic!("{what}: source tier 3, {e:?}"));
    let creases = rod_creases(body);
    assert_eq!(creases.len(), 2, "{what}: the D's two creases");
    match fillet_edges(body, &creases, ROD_FILLET, tol()).map_err(|e| e.error) {
        Err(BlendError::RingClearance { face, margin }) => {
            assert_eq!(
                margin.sign,
                Sign::Negative,
                "{what}: definite, got {margin}"
            );
            let f = body
                .get_face(face)
                .expect("the refusal names a source face");
            assert!(
                !f.rings.is_empty(),
                "{what}: the face named is a cap carrying the bore"
            );
        }
        Err(other) => panic!("{what}: expected RingClearance at the cap, got {other:?}"),
        Ok(out) => panic!(
            "{what}: the carve returned a body keeping the bore's cycle on a cap that no \
             longer covers it ({} faces)",
            out.body.faces().count()
        ),
    }
}

/// **A bore wholly inside the removed sliver refuses** rather than
/// being left on the cap outside the region it bounds. The bore at
/// `(0.297, 0.395)` is `0.003` from the flat, `0.0033` from the wall
/// and `≈ 0.1085` from `c`: every point of it is in the material the
/// band removes.
#[test]
fn a_bore_inside_the_d_rods_removed_sliver_refuses_ring_clearance() {
    assert_cap_ring_refusal(&bored_d_rod(0.297, 0.395, 0.002), "bore in the sliver");
}

/// **A bore straddling the cut-off arc refuses**: centred ON the arc,
/// on the ray from `c` towards `V`, so it is part removed and part kept.
#[test]
fn a_bore_across_the_d_rods_cut_off_arc_refuses_ring_clearance() {
    let (c, v) = upper_corner(ROD_FILLET);
    let (dx, dy) = (v.0 - c.0, v.1 - c.1);
    let n = dx.hypot(dy);
    let (x, y) = (c.0 + ROD_FILLET * dx / n, c.1 + ROD_FILLET * dy / n);
    assert_cap_ring_refusal(&bored_d_rod(x, y, 0.004), "bore across the arc");
}

/// **A bore definitely clear of the sliver carves as before**, and the
/// bore is carried through untouched: `ΔV = −2·A·L` exactly, the D-rod's
/// own closed form. Two clear readings: a bore at the axis, beyond the
/// annulus's reach (`‖c‖ − a − ρ > 0`), and one INSIDE the ball's
/// section at `c` itself (`r − a > 0`), which is kept material.
#[test]
fn a_bore_clear_of_the_d_rods_sliver_carves_at_the_closed_form() {
    let (c, _) = upper_corner(ROD_FILLET);
    for (x, y, a, what) in [
        (0.0, 0.0, 0.1, "bore at the axis"),
        (c.0, c.1, 0.05, "bore inside the ball's section"),
    ] {
        let body = bored_d_rod(x, y, a);
        validate_geometric(&body, tol()).unwrap_or_else(|e| panic!("{what}: tier 3, {e:?}"));
        let creases = rod_creases(&body);
        let vol0 = volume(&body);
        let out = fillet_edges(&body, &creases, ROD_FILLET, tol())
            .unwrap_or_else(|e| panic!("{what}: both creases carve, got {e}"));
        validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("{what}: tier 3, {e:?}"));
        let dv = volume(&out.body) - vol0;
        let want = -2.0 * rod_section_cut(ROD_R, ROD_FLAT, ROD_FILLET) * ROD_L;
        assert!((dv - want).abs() < 1e-12, "{what}: ΔV {dv} vs {want}");
    }
}
