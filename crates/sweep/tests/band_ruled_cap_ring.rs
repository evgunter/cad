//! **A cap edge beside a convex ruled crease's cut-off.** The ruled
//! band removes, from each transverse cap, the SLIVER between the
//! cut-off arc (radius `r` about the spine's crossing `c`) and the
//! crease's old vertex `V`. An edge of the cap lying in that sliver — a
//! bore's ring, or a notch in the cycle the cut runs in — would be left
//! on the cap outside the region it bounds, or crossed by the arc, so
//! every edge of the cap other than the two rims the cut shortens is
//! metered, over its own window, against a region enclosing the sliver
//! before any carve: `r ≤ ‖p − c‖ ≤ ρ` (`ρ` the farthest the sliver
//! reaches from `c`, which on this rod is `‖V − c‖`), cut down to the
//! half-plane `(p − c)·(V − c) ≥ floor` the sliver lies in.
//!
//! The fixture is the D-rod (`test_support::rod_d_profile_at`'s D at
//! [`ROD_FLAT`]) with a second profile loop through it, or a channel in
//! its outline. Its upper crease's ball centre is `c = (0.2, √0.12)`
//! and its old vertex `V = (0.3, 0.4)`, so at [`ROD_FILLET`] the
//! annulus is `0.1 ≤ ‖p − c‖ ≤ ‖V − c‖ ≈ 0.1134`.
//!
//! The keyhole twin — a bore beside a cut-off that runs in a cap RING —
//! is `review_band_ruled_ring_probes`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Sign, Tol};
use profile::{ProfileLoop, SketchPlane, test_support::bulge_loop};
use sweep::blend::{BlendError, Convexity, fillet_edges};
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
    match fillet_edges(body, &creases, ROD_FILLET, tol()).map_err(|e| {
        let text = e.error.to_string();
        (e.error, text)
    }) {
        Err((
            BlendError::RingClearance {
                face,
                chain,
                margin,
            },
            text,
        )) => {
            assert_eq!(
                chain,
                Convexity::Convex,
                "a cap meter refuses only beside a convex crease"
            );
            assert!(
                text.contains(
                    "lies in the part of a face the blend cuts away with the material it removes"
                ),
                "the sentence says the edge is cut away with the sliver: {text}"
            );
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
        Err((other, _)) => panic!("{what}: expected RingClearance at the cap, got {other:?}"),
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

/// A closed polygonal profile loop through `pts`.
fn poly(pts: &[(f64, f64)]) -> ProfileLoop<f64> {
    bulge_loop(pts.iter().map(|&(x, y)| (Point2::new(x, y), 0.0)).collect())
}

/// The D-rod's own outline.
fn d_loop() -> ProfileLoop<f64> {
    let c = rod_chord_at(ROD_FLAT);
    bulge_loop(vec![
        (Point2::new(ROD_FLAT, c.half), c.wall_bulge),
        (Point2::new(ROD_FLAT, -c.half), 0.0),
    ])
}

/// Both creases of `body` carve at [`ROD_FILLET`], tier 3 accepts the
/// result, and `ΔV = −2·A·L` exactly: whatever else the caps carry is
/// carried through untouched.
fn assert_carves_at_the_closed_form(body: &Body<f64>, what: &str) {
    validate_geometric(body, tol()).unwrap_or_else(|e| panic!("{what}: source tier 3, {e:?}"));
    let creases = rod_creases(body);
    assert_eq!(creases.len(), 2, "{what}: the D's two creases");
    let vol0 = volume(body);
    let out = fillet_edges(body, &creases, ROD_FILLET, tol())
        .unwrap_or_else(|e| panic!("{what}: both creases carve, got {e}"));
    validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("{what}: tier 3, {e:?}"));
    let dv = volume(&out.body) - vol0;
    let want = -2.0 * rod_section_cut(ROD_R, ROD_FLAT, ROD_FILLET) * ROD_L;
    assert!((dv - want).abs() < 1e-12, "{what}: ΔV {dv} vs {want}");
}

/// **A square drive hole on the D-shaft's axis carves.** Its `x = +h`
/// side lies on a line that passes within the sliver's reach of `c`
/// (at `h = 0.2` it runs through `c`'s `x`), so a meter reading the
/// side's whole carrier line refused all three; the SIDE, a segment
/// ending at `y = ±h`, stays short of the corner, and each edge is
/// metered over its own window.
#[test]
fn a_square_hole_on_the_d_shafts_axis_carves_at_the_closed_form() {
    for h in [0.1, 0.15, 0.2] {
        let sq = poly(&[(-h, -h), (h, -h), (h, h), (-h, h)]);
        let body = extruded(SketchPlane::xy(), vec![d_loop(), sq], ROD_L, tol());
        assert_carves_at_the_closed_form(&body, &format!("square hole h = {h}"));
    }
}

/// **A bore in the annulus but away from the corner carves**: behind
/// the ball centre (on the ray from `V` through `c`) and beside it
/// (either way across that ray), each `0.106` from `c`. All three lie
/// inside the annulus `r ≤ ‖p − c‖ ≤ ‖V − c‖` the sliver lies in, in
/// kept material; the half-plane `(p − c)·(V − c) ≥ floor` the sliver
/// also lies in puts them out of reach. The two beside `c` read
/// `(p − c)·(V − c) ≈ 0`, which clears only if `floor` is the sliver's
/// own (`≈ 0.085·‖V − c‖` here) — so they fail if the cut-off arc were
/// taken the wrong way round the section circle, whose far side would
/// drag `floor` down to `−r`.
#[test]
fn a_bore_in_the_annulus_away_from_the_corner_carves_at_the_closed_form() {
    let (c, v) = upper_corner(ROD_FILLET);
    let n = (v.0 - c.0).hypot(v.1 - c.1);
    let (ux, uy) = ((v.0 - c.0) / n, (v.1 - c.1) / n);
    let a = 0.003;
    assert!(
        0.106 - a > ROD_FILLET && 0.106 + a < n,
        "inside the annulus"
    );
    for (dx, dy, what) in [
        (-ux, -uy, "behind the ball centre"),
        (-uy, ux, "beside the ball centre, wallward"),
        (uy, -ux, "beside the ball centre, flatward"),
    ] {
        let (x, y) = (c.0 + 0.106 * dx, c.1 + 0.106 * dy);
        assert_carves_at_the_closed_form(&bored_d_rod(x, y, a), what);
    }
}

/// **A channel in the D's own outline reaching into the sliver
/// refuses.** An L-shaped channel is cut in from the flat below the
/// upper foot, up past the ball's section, and along to a tip at
/// `x = 0.295, y ∈ [0.385, 0.39]` — inside the sliver, `≈ 0.1045` from
/// `c`. Its edges belong to the cycle the cut-off runs in, so the arc
/// would be `mef`'d across them; each is metered like any other cap
/// edge, and the tip's refuses at the cap.
#[test]
fn a_channel_in_the_cut_cycle_reaching_into_the_sliver_refuses_ring_clearance() {
    let c = rod_chord_at(ROD_FLAT);
    let d = bulge_loop(vec![
        (Point2::new(ROD_FLAT, c.half), c.wall_bulge),
        (Point2::new(ROD_FLAT, -c.half), 0.0),
        (Point2::new(ROD_FLAT, 0.2), 0.0),
        (Point2::new(0.15, 0.2), 0.0),
        (Point2::new(0.15, 0.39), 0.0),
        (Point2::new(0.295, 0.39), 0.0),
        (Point2::new(0.295, 0.385), 0.0),
        (Point2::new(0.16, 0.385), 0.0),
        (Point2::new(0.16, 0.21), 0.0),
        (Point2::new(ROD_FLAT, 0.21), 0.0),
    ]);
    let body = extruded(SketchPlane::xy(), vec![d], ROD_L, tol());
    validate_geometric(&body, tol()).expect("source tier 3");
    let creases = rod_creases(&body);
    assert_eq!(creases.len(), 2, "the D's two creases");
    match fillet_edges(&body, &creases, ROD_FILLET, tol()).map_err(|e| {
        let text = e.error.to_string();
        (e.error, text)
    }) {
        Err((
            BlendError::RingClearance {
                face,
                chain,
                margin,
            },
            text,
        )) => {
            assert_eq!(
                chain,
                Convexity::Convex,
                "a cap meter refuses only beside a convex crease"
            );
            assert!(
                text.contains(
                    "lies in the part of a face the blend cuts away with the material it removes"
                ),
                "the sentence says the edge is cut away with the sliver: {text}"
            );
            assert_eq!(margin.sign, Sign::Negative, "definite, got {margin}");
            let f = body
                .get_face(face)
                .expect("the refusal names a source face");
            assert!(
                f.rings.is_empty()
                    && matches!(
                        body.get_surface(f.surface),
                        Some(geom::Surface::Plane { normal, .. }) if normal.z.abs() > 0.5
                    ),
                "the face named is a cap, the channel in its outer cycle"
            );
        }
        Err((other, _)) => panic!("expected RingClearance at the cap, got {other:?}"),
        Ok(out) => panic!(
            "the cut-off arc was mef'd across the channel's edges ({} faces)",
            out.body.faces().count()
        ),
    }
}
